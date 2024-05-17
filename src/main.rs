use std::{fs, env};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use microbench::{self, Options};
use bitvec::prelude::*;

use crate::parser::*;
use crate::utils::*;
use crate::tokenizer::*;
use crate::taxonlabels::*;
use crate::splits::*;

pub mod parser;
pub mod utils;
pub mod tokenizer;
pub mod tree;
pub mod taxonlabels;
pub mod splits;





fn main() -> io::Result<()> {
    //let contents = fs::read_to_string("primates.tre").expect("should have been able to readfile");
    //let stripped_contents = stripcomments(&contents); 
//    let stripped_contents = fs::read_to_string("ungulates.tre").expect("should have been able to readfile");



    //println!("With text: \n {stripped_contents}");

    let s = "((((A:0.5,B:0.5):1.5,C:1.5):0.5,(D:2,E:0.5):0.5):2.5,F:2.5);";
    //let s = re.replace_all(string_with_comments, "");
    //let s = stripped_contents;
 //   let tokens = tokenize(&stripped_contents);
  //  let root = parse_newick(&tokens);
   // let all_taxa = taxon_labels(&root);
   // println!("taxon labels: \t {:?}", taxon_labels(&root));

    //let n_tips = taxon_labels(&root).len();
 //   println!("number of taxa: {}", n_tips);

   // let mut splits: Vec<BitVec> = Vec::new();
  //  postorder_splits(&mut splits, &all_taxa, &root);

    //for split in &splits{
    //    println!("split: \t {:?}", &split);
   // }

    

    let mut h: HashMap<BitVec, u64> = HashMap::new();
    //for split in &splits{
    //    *h.entry(split.clone()).or_insert(0) += 1;
    //}



    //BufRead

    let f = File::open("primates_cytb_JC_run_1.trees")?;
    let f = BufReader::new(f);

    for (i, line) in f.lines().enumerate(){
        if i > 0 {
            let line_string = line.unwrap();
          //  println!("{}", &line_string);
            let newickstring = find_newick_string(line_string);
            //println!("{}", &newickstring);

            let stripped_contents = stripcomments(&newickstring); 
            //println!("stripped: \t {:?}", &stripped_contents);
            let tokens = tokenize(&stripped_contents);
            println!("tokens: \t {:?}", &tokens);
            let root = parse_newick(&tokens);
            let all_taxa = taxon_labels(&root);
            println!("all taxa: \t {:?}", &all_taxa);

            // calculate the splits
            let mut splits: Vec<BitVec> = Vec::new();
            postorder_splits(&mut splits, &all_taxa, &root);

            // add the splits to the dictionary
            for split in &splits{
                *h.entry(split.clone()).or_insert(0) += 1;
            }

            for split in splits{
               println!("{:?}", split); 
            }
        }
    }
     
    println!("summary hash map: \t");
  //  for (key, value) in h{
 //       println!("key: {:?}, \t val: {}", &key, &value);
//    }

    //let options = Options::default();
    //microbench::bench(&options, "collect leaf labels", || taxon_labels(&root));
    Ok(())
}


