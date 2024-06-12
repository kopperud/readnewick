use std::rc::Rc;
use bitvec::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use fasthash::city::Hash32;

use crate::tree::*;
use crate::taxonlabels::*;

pub fn root_splits(
    splits: &mut Vec<BitVec>, 
    //all_taxa: &Vec<String>,
    //taxa_map: &BTreeMap<String, usize>,
    taxa_map: &HashMap<String, usize, Hash32>,
    n_taxa: &usize,
    node: &Rc<Node>
    ){
    let children = node.children.borrow();
   
    if !children.is_empty(){
        for child in children.iter(){
            //postorder_splits(splits, all_taxa, &child_branch.outbounds.borrow());
            postorder_splits(splits, taxa_map, n_taxa, child);
        }
    }
}
pub fn postorder_splits(
    splits: &mut Vec<BitVec>, 
    //all_taxa: &Vec<String>,
    //taxa_map: &BTreeMap<String, usize>,
    taxa_map: &HashMap<String, usize, Hash32>,
    n_taxa: &usize,
    node: &Rc<Node>
    ) -> Vec<String>
{
    let children = node.children.borrow();

    let mut split_taxa: Vec<String> = Vec::new();

    if children.is_empty(){
        split_taxa.push(node.label.clone());
    }else{
 
        for child in children.iter(){
            let mut subtree_taxa = postorder_splits(splits, taxa_map, n_taxa, child);
            split_taxa.append(&mut subtree_taxa);
        }   

        //if !children.is_empty(){
        //    let split_taxa = taxon_labels(node);
        let mut split: BitVec = BitVec::repeat(false, *n_taxa);

        /*for taxon in all_taxa {
            let x = split_taxa.contains(taxon);
            split.push(x);
        }*/
        for taxon in split_taxa.iter(){
            let idx = taxa_map.get(taxon).unwrap();
            split.set(*idx, true);
        }

        splits.push(split);

    }
    split_taxa
}
