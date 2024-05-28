use std::rc::Rc;
use bitvec::prelude::*;

use crate::tree::*;
use crate::taxonlabels::*;

pub fn root_splits(
    splits: &mut Vec<BitVec>, 
    all_taxa: &Vec<String>,
    node: &Rc<Node>
    ){
    let children = node.children.borrow();
   
    if !children.is_empty(){
        for child in children.iter(){
            //postorder_splits(splits, all_taxa, &child_branch.outbounds.borrow());
            postorder_splits(splits, all_taxa, &child);
        }
    }
}
pub fn postorder_splits(
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

        for child in children.iter(){
            postorder_splits(splits, all_taxa, &child);
        }
    }
}
