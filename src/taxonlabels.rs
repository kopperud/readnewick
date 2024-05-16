use crate::tree::*;
use std::rc::{Rc, Weak};

pub fn taxon_labels(root: &Rc<Node>) -> Vec<String> {
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


