use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::{HashMap, HashSet};
use bitvec::prelude::*;
use indicatif::ProgressBar;
use clap::{Command, Arg, ArgAction};
use csv::Writer;
use fasthash::city::Hash32;
use fasthash::farm::Hash64;

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
    let mut global_splits: HashSet<BitVec, Hash64> = HashSet::with_hasher(Hash64);

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

    for filename in filenames.iter(){

        let file = File::open(filename)?;
        let n_lines = count_lines(&file).unwrap();

        let file = File::open(filename)?;
        let f = BufReader::new(&file);

        let n_trees = (n_lines - 1).try_into().expect("expected to be able to convert usize to u64");
        let bar = ProgressBar::new(n_trees);

        let lines = f.lines();
        let mut h: HashMap<BitVec, u64, Hash64> = HashMap::with_hasher(Hash64);
        let mut n_trees = 0.0;
        
        for (i, line) in lines.enumerate(){
            if (i > 0) & ((i as f64) > ((burnin) * n_lines as f64)) {
                let line_string: String = line.unwrap();
                let root = parse_tree(line_string);

                // calculate the splits
                let mut splits: Vec<BitVec> = Vec::new();
                root_splits(&mut splits, &taxa_map, &n_taxa, &root);

                // add the splits to the dictionary
                for split in splits.into_iter(){
                    *h.entry(split.clone()).or_insert(0) += 1;
                    global_splits.insert(split);
                }
                n_trees += 1.0;
            }
            bar.inc(1);
        }
        bar.finish();
         
        // calculate split frequencies
        let mut split_frequencies: HashMap<BitVec, f64, Hash64> = HashMap::with_hasher(Hash64);
        for (key, value) in h{
            let split_occurrences = value as f64;
            let split_frequency = split_occurrences / n_trees;
            *split_frequencies.entry(key.clone()).or_insert(0.0) = split_frequency;
        }
        split_frequencies_per_file.push(split_frequencies);
    }
    eprintln!();
    
    // add in the zero splits 
    //for split_frequencies in &mut split_frequencies_per_file{
    for split_frequencies in split_frequencies_per_file.iter_mut(){
        for split in &global_splits{
            if !split_frequencies.contains_key(split){
                *split_frequencies.entry(split.clone()).or_insert(0.0) = 0.0;
            }
        }
    }

    if has_outname{
        let output_filename = cmd
            .get_one::<String>("output")
            .unwrap();
        let mut wtr = Writer::from_path(output_filename)?;

        let mut header = vec!["split"];
        for filename in filenames{
            header.push(filename);
        }

        wtr.write_record(header)?;
        for split in global_splits.iter(){
            let mut line: Vec<String> = vec![];
            let splitstr = format!("{:b}", split)
                .replace(", ", "");
            line.push(splitstr);

            for split_frequencies in split_frequencies_per_file.iter(){
                let sf = split_frequencies[split].to_string();
                line.push(sf);
            }
            wtr.write_record(line)?;
        }

    }else{
        // print summary to stdout
        println!("split \t frequency");
        for split in global_splits.iter(){
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


