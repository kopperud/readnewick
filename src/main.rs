use std::{fs, env};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use microbench::{self, Options};
use bitvec::prelude::*;
use indicatif::ProgressBar;
use std::convert::TryFrom;


use crate::parser::*;
use crate::utils::*;
use crate::tokenizer::*;
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


pub fn parse_tree(contents: String) -> Rc<Node> {
    let newickstring = find_newick_string(contents);

    let stripped_contents = stripcomments(&newickstring); 
    let tokens = tokenize(&stripped_contents);
    let root = parse_newick(&tokens);

    return root
}



fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().skip(1).collect();
    dbg!(&args);



    let filenames = args;

    // read first tree
    // save the taxon names
    // assume they are equal for all samples
    let root = parse_tree(second_line);
    let all_taxa = taxon_labels(&root);

    for filename in filenames{
        let filename = "primates_cytb_JC_run_1.trees";

        let file = File::open(&filename)?;
        let n_lines = count_lines(&file).unwrap();

        let file = File::open(&filename)?;
        let f = BufReader::new(&file);
        let count: u32 = n_lines as u32;

        let bar = ProgressBar::new(n_lines.try_into().unwrap());

        let lines = f.lines();
        let mut h: HashMap<BitVec, u64> = HashMap::new();
        let mut n_trees = 1.0;
        
        for (i, line) in lines.enumerate(){
            if i > 0 {
                let line_string = line.unwrap();
                let root = parse_tree(line_string);
                //let all_taxa = taxon_labels(&root);
                //println!("all taxa: \t {:?}", &all_taxa);

                // calculate the splits
                let mut splits: Vec<BitVec> = Vec::new();
                postorder_splits(&mut splits, &all_taxa, &root);

                // add the splits to the dictionary
                for split in &splits{
                    *h.entry(split.clone()).or_insert(0) += 1;
                }
                n_trees += 1.0;
                bar.inc(1);
            }
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

        println!("split frequencies across posterior sample: \t");
        println!("split \t frequency");
        for (key, value) in &split_frequencies{
            println!("{} \t {}", &key.to_string().replace(", ", ""), &value);
        }

    }
        //let options = Options::default();
    //microbench::bench(&options, "collect leaf labels", || taxon_labels(&root));
    Ok(())
}


