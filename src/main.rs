use std::{fs, env};
use std::io::{self, prelude::*, BufReader};
use std::rc::{Rc, Weak};
use std::collections::{HashSet, HashMap};
use microbench::{self, Options};
use regex::Regex;
use bitvec::prelude::*;

use crate::parser::*;
use crate::utils::*;
use crate::tokenizer::*;
use crate::tree::*;

pub mod parser;
pub mod utils;
pub mod tokenizer;
pub mod tree;



fn taxon_labels(root: &Rc<Node>) -> Vec<String> {
    let mut taxa: Vec<String> = vec![];

    taxon_labels_po(&mut taxa, root);
    return taxa
}

fn taxon_labels_po(taxa: &mut Vec<String>, node: &Rc<Node>){
    let children = node.children.borrow();

    if children.is_empty(){
        taxa.push(node.label.clone());
    }else{
        for child_branch in children.iter(){
            taxon_labels_po(taxa, &child_branch.outbounds.borrow());
        }
    }
}

fn postorder_splits(
    splits: &mut Vec<BitVec>, 
    all_taxa: &Vec<String>,
    node: &Rc<Node>
    ){
    let children = node.children.borrow();

    if !children.is_empty(){
        let split_taxa = taxon_labels(&node);
        let mut split: BitVec = BitVec::new();

        for taxon in all_taxa {
            let x = split_taxa.contains(&taxon);
            split.push(x);
        }

        splits.push(split);

        for child_branch in children.iter(){
            postorder_splits(splits, all_taxa, &child_branch.outbounds.borrow());
        }
    }
}


fn main() {
    //let contents = fs::read_to_string("primates.tre").expect("should have been able to readfile");
    //let stripped_contents = stripcomments(&contents); 
    let stripped_contents = fs::read_to_string("ungulates.tre").expect("should have been able to readfile");


    //println!("With text: \n {stripped_contents}");

    let s = "((((A:0.5,B:0.5):1.5,C:1.5):0.5,(D:2,E:0.5):0.5):2.5,F:2.5);";
    //let s = re.replace_all(string_with_comments, "");
    //let s = stripped_contents;
    let tokens = tokenize(&stripped_contents);

    for token in tokens.iter(){
        let isp = token == ",";
        println!("{token} \t, is comma = {}", isp);
    }
  
    let root = parse_newick(&tokens);
    let all_taxa = taxon_labels(&root);

    println!("taxon labels: \t {:?}", taxon_labels(&root));

    let options = Options::default();
    //microbench::bench(&options, "collect leaf labels", || taxon_labels(&root));

    let n_tips = taxon_labels(&root).len();
    println!("number of taxa: {}", n_tips);

    let mut splits: Vec<BitVec> = Vec::new();

    postorder_splits(&mut splits, &all_taxa, &root);

    for split in &splits{
        println!("split: \t {:?}", &split);
    }



    let mut h: HashMap<BitVec, u64> = HashMap::new();
    for split in &splits{
        *h.entry(split.clone()).or_insert(0) += 1;
    }
    println!("summary hash map: \t");
    for (key, value) in h{
        println!("key: {:?}, \t val: {}", &key, &value);
    }

}


