use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::sync::{Mutex,Arc};
use std::collections::{HashMap, HashSet};
use bitvec::prelude::*;
use indicatif::ProgressBar;
use indicatif::ParallelProgressIterator;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use clap::{Command, Arg, ArgAction};
use csv::Writer;
use fasthash::city::Hash32;
use fasthash::farm::Hash64;
use rayon::prelude::*;

use crate::parser::*;
use crate::taxonlabels::*;
use crate::splits::*;
use crate::linecount::*;

pub mod parser;
pub mod utils;
pub mod tokenizer;
pub mod tree;
pub mod taxonlabels;
pub mod splits;
pub mod linecount;


fn main() -> io::Result<()> {

    let cmd = Command::new("splitfrequencies")
        .about("this program reads tree files")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .num_args(1..=16)
            .value_name("FILENAMES")
            .required(true)
            .help("tree files used for the program")
            .action(ArgAction::Append)
            .value_parser(clap::value_parser!(String)))
        //.get_matches_from(vec!["splitfrequencies", "-i", "file1.tre", "file2.tre"]);
        //
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .action(ArgAction::Set)
            .value_name("FILENAME")
            .help("file name for the csv output"))
        .arg(Arg::new("burnin")
            .short('b')
            .long("burnin")
            .value_name("BURNIN")
            .help("the fraction of sampled (starting from the top) to be discarded")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(f64))
            .default_value("0.1"))
           //.value_parser(clap::value_parser!(char)))
        //.about("hello")
       .get_matches();



    let mut innames: Vec<Vec<&String>> = cmd
        .get_occurrences("input")
        .unwrap()
        .map(Iterator::collect)
        .collect();

    let burnin = cmd
        .get_one("burnin")
        .unwrap();

    let has_outname = cmd
        .contains_id("output");

    if has_outname{
        // write to file
        let output_filename = cmd
            .get_one::<String>("output")
            .unwrap();
        eprintln!("output file: \t {}", output_filename);
    }

    let filenames = innames.remove(0);
    eprintln!("input files: \t {:?}", &filenames);
    let mut global_splits: Arc<Mutex<HashSet<BitVec, Hash64>>> = Arc::new(Mutex::new(HashSet::with_capacity_and_hasher(500000, Hash64)));

    // read first tree
    // save the taxon names
    // assume they are equal for all samples
    let first_filename = filenames.first().expect("no tree path arguments");
    let reader = BufReader::new(File::open(first_filename).expect("cannot open file"));
    let second_line = reader.lines()
        .nth(1)
        .expect("input is not two lines long")
        .expect("could not read second line");

    let root = parse_tree(second_line.clone());
    let all_taxa = taxon_labels(&root);
    let n_taxa = all_taxa.len();
    
    let mut taxa_map = HashMap::with_hasher(Hash32);

    for (i, taxon) in all_taxa.iter().enumerate(){
        taxa_map.insert(taxon.to_owned(), i);
    }
        

    let mut split_frequencies_per_file = vec![];

    for (file_index, filename) in filenames.iter().enumerate(){

        println!("calculating splits for file {}", file_index);

        let file = File::open(filename)?;
        let n_lines = count_lines(&file).unwrap();

        let n_trees: u64 = (n_lines - 1)
            .try_into()
            .expect("expected to be able to convert usize to u64");


        let n_skip = (burnin * n_lines as f64).round() as usize;
        //println!("n_skip: {}", n_skip);
        let n_keep = n_trees - (n_skip as u64);

        let mut this_file_map: Arc<Mutex<HashMap<BitVec, u64, Hash64>>> = Arc::new(Mutex::new(HashMap::with_capacity_and_hasher(500000, Hash64)));

            let file = File::open(filename)?;
            let f = BufReader::new(&file);

            let lines = f
                .lines()
                .skip(n_skip);
           
            let this_file_splits: Vec<Vec<BitVec>> = lines
               .par_bridge()
               .progress_count(n_keep)
               .map(|line| -> Vec<BitVec> {
                    let line_string: String = line.unwrap();
                    let root = parse_tree(line_string);

                    // calculate the splits
                    let mut splits: Vec<BitVec> = Vec::new();
                    root_splits(&mut splits, &taxa_map, &n_taxa, &root);

                    splits
                })
               .collect();

        /*
        let mut this_file_splits: Vec<Vec<BitVec>> = Vec::new();
        let file = File::open(filename)?;
        let f = BufReader::new(&file);
        let lines = f
            .lines()
            .skip(n_skip);


        let bar = ProgressBar::new(n_trees);
        bar.inc(n_skip as u64);

        for line in lines{
            let line_string: String = line.unwrap();
            let root = parse_tree(line_string);

            // calculate the splits
            let mut splits: Vec<BitVec> = Vec::new();
            root_splits(&mut splits, &taxa_map, &n_taxa, &root);

            this_file_splits.push(splits);
        }
        bar.finish();
        */

        eprintln!("inserting splits in hash table for file {}", file_index);
        let mut n_processed = 0.0;
        for splits in this_file_splits.into_iter(){
                // add the splits to the dictionary
                for split in splits.into_iter(){
                    //let key = split;
                    let hm: Arc<Mutex<HashMap<BitVec, u64, Hash64>>> = Arc::clone(&this_file_map);
                    let mut h = hm.lock().unwrap();
                    *h.entry(split.clone()).or_insert(0) += 1;

                    //*this_file_splits.entry(split.clone()).or_insert(0) += 1;
                    //
                    //*h.entry(key).or_insert(0) += 1;
                    //global_splits.insert(key);
                    let gs = Arc::clone(&global_splits);
                    gs.lock().unwrap().insert(split);
                }
                n_processed += 1.0;
            //bar.inc(1);
        } 
         
        // calculate split frequencies
        println!("calculating split frequencies for file {}", file_index);
        let mut split_frequencies: HashMap<BitVec, f64, Hash64> = HashMap::with_hasher(Hash64);
        let hm: Arc<Mutex<HashMap<BitVec, u64, Hash64>>> = Arc::clone(&this_file_map);
        let h = hm.lock().unwrap();
        for (key, value) in h.iter(){
            let split_occurrences = *value as f64;
            let split_frequency = split_occurrences / n_processed;
            *split_frequencies.entry(key.clone()).or_insert(0.0) = split_frequency;
        }
        split_frequencies_per_file.push(split_frequencies);
    }
    eprintln!();
    
    // add in the zero splits 
    //for split_frequencies in &mut split_frequencies_per_file{
    println!("adding the split frequencies that are zero");
    for split_frequencies in split_frequencies_per_file.iter_mut(){
        let gs = Arc::clone(&global_splits);
        let g = gs.lock().unwrap();
        let iter = g.iter();
        for split in iter{
            if !split_frequencies.contains_key(split){
                *split_frequencies.entry(split.clone()).or_insert(0.0) = 0.0;
            }
        }
    }

    if has_outname{
        println!("writing to file");
        let output_filename = cmd
            .get_one::<String>("output")
            .unwrap();
        let mut wtr = Writer::from_path(output_filename)?;

        let mut header = vec!["split"];
        for filename in filenames{
            header.push(filename);
        }

        wtr.write_record(header)?;

        let gs = Arc::clone(&global_splits);
        let g = gs.lock().unwrap();
        let iter = g.iter();

        for split in iter{
            let mut line: Vec<String> = vec![];
            let splitstr = format!("{:b}", split)
                .replace(", ", "");
            line.push(splitstr);

            for split_frequencies in split_frequencies_per_file.iter(){
                //let sf = split_frequencies[split].to_string();
                let sf = format!("{}", split_frequencies[split]);
                line.push(sf);
            }
            wtr.write_record(line)?;
        }

    }else{
        // print summary to stdout
        println!("split \t frequency");
        let gs = Arc::clone(&global_splits);
        let g = gs.lock().unwrap();
        let iter = g.iter();

        for split in iter{
            let splitstr = format!("{:b}", split)
                .replace(", ", "");

            print!("{} \t ", splitstr);

            for split_frequencies in &split_frequencies_per_file{
                print!("{:.6} \t ", split_frequencies[split]);
            }
            println!();
        }
    }

    Ok(())
}


