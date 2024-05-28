use std::{fs, env}; use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::{HashMap, HashSet};
use microbench::{self, Options};
use bitvec::prelude::*;
use indicatif::ProgressBar;
use std::convert::TryFrom;
use std::rc::Rc;
use regex::Regex;
use clap::{Parser, Command, Arg, ArgAction, ArgGroup};



use crate::parser::*;
use crate::utils::*;
use crate::tokenizer::*;
use crate::taxonlabels::*;
use crate::splits::*;
use crate::linecount::*;
use crate::tree::*;

pub mod parser;
pub mod utils;
pub mod tokenizer;
pub mod tree;
pub mod taxonlabels;
pub mod splits;
pub mod linecount;


pub fn parse_tree(contents: String) -> Rc<Node> {
    let newickstring = find_newick_string(contents);

    let stripped_contents = stripcomments(&newickstring); 
    let tokens = tokenize(&stripped_contents);
    let root = parse_newick(&tokens);

    return root
}



fn main() -> io::Result<()> {

    let cmd = Command::new("readnewick")
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
        //.get_matches_from(vec!["readnewick", "-i", "file1.tre", "file2.tre"]);
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
            .about("hello")
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
    let mut global_splits: HashSet<BitVec> = HashSet::new();

    // read first tree
    // save the taxon names
    // assume they are equal for all samples
    let first_filename = filenames.get(0).expect("no tree path arguments");
    let reader = BufReader::new(File::open(first_filename).expect("cannot open file"));
    let second_line = reader.lines()
        .nth(1)
        .expect("input is not two lines long")
        .expect("could not read second line");

    let root = parse_tree(second_line.clone());
    let all_taxa = taxon_labels(&root);

    let mut split_frequencies_per_file = vec![];

    for filename in filenames{

        let file = File::open(&filename)?;
        let n_lines = count_lines(&file).unwrap();

        let file = File::open(&filename)?;
        let f = BufReader::new(&file);

        let bar = ProgressBar::new(n_lines.try_into().unwrap());

        let lines = f.lines();
        let mut h: HashMap<BitVec, u64> = HashMap::new();
        let mut n_trees = 0.0;
        
        for (i, line) in lines.enumerate(){
            if (i > 0) & ((i as f64) > ((burnin) * n_lines as f64)) {
                let line_string: String = line.unwrap();
                let root = parse_tree(line_string);
                //let all_taxa = taxon_labels(&root);
                //println!("all taxa: \t {:?}", &all_taxa);

                // calculate the splits
                let mut splits: Vec<BitVec> = Vec::new();
                postorder_splits(&mut splits, &all_taxa, &root);

                // add the splits to the dictionary
                for split in &splits{
                    *h.entry(split.clone()).or_insert(0) += 1;
                    
                    global_splits.insert(split.clone());
                }
                n_trees += 1.0;
            }
            bar.inc(1);
        }
        bar.finish();
         
        //println!("summary hash map: \t");
        //for (key, value) in &h{
        //    println!("key: {:?}, \t val: {}", &key, &value);
        //}

        // calculate split frequencies
        let mut split_frequencies: HashMap<BitVec, f64> = HashMap::new();
        for (key, value) in h{
            let split_occurrences = value as f64;
            let split_frequency = split_occurrences / n_trees;
            *split_frequencies.entry(key.clone()).or_insert(0.0) = split_frequency;
        }
        split_frequencies_per_file.push(split_frequencies);
    }
    eprintln!("");
    
    // add in the zero splits 
    for split_frequencies in &mut split_frequencies_per_file{
        for split in &global_splits{

            if !split_frequencies.contains_key(split){
                *split_frequencies.entry(split.clone()).or_insert(0.0) = 0.0;
            }
        }
    }

    if has_outname{
        // write to file

    }else{
        // print summary to stdout
        println!("split \t frequency");
        for split in &global_splits{
            print!("{} \t ", &split.to_string().replace(", ", ""));

            for split_frequencies in &split_frequencies_per_file{
                print!("{:.6} \t ", split_frequencies[split]);
            }
            print!("\n");
        }
    }

/*
    for split_frequencies in split_frequencies_per_file{
        println!("split frequencies across posterior sample: \t");
        println!("split \t frequency");
        for (key, value) in &split_frequencies{
            println!("{} \t {}", &key.to_string().replace(", ", ""), &value);
        }

    }
*/
            //let options = Options::default();
    //microbench::bench(&options, "collect leaf labels", || taxon_labels(&root));

    /*
    let newickstring = find_newick_string(second_line.clone());

    let stripped_contents = stripcomments(&newickstring); 
    let tokens = tokenize(&stripped_contents);
    let root = parse_newick(&tokens);

    let options = Options::default();
    microbench::bench(&options, "find newick string", || find_newick_string(second_line.clone()));
    microbench::bench(&options, "strip comments", || stripcomments(&newickstring));
    microbench::bench(&options, "tokenize", || tokenize(&stripped_contents));
    microbench::bench(&options, "parse tokens", || parse_newick(&tokens));
    microbench::bench(&options, "all steps combined", || parse_tree(second_line.clone()));

    
    microbench::bench(&options, "create a regex", || Regex::new(r"\[.*?\]").unwrap());
    */

    Ok(())
}


