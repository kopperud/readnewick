use crate::tree::*;
use std::rc::Rc;

pub fn taxon_labels(
    node: &Rc<Node>
    ) -> Vec<String> 
{
    let mut taxa: Vec<String> = vec![];

    taxon_labels_po(&mut taxa, node);
    taxa
}

fn taxon_labels_po(
    taxa: &mut Vec<String>, 
    node: &Rc<Node>
    )
{
    let children = node.children.borrow();

    if children.is_empty(){
        taxa.push(node.label.clone());
    }else{
        for child in children.iter(){
            taxon_labels_po(taxa, child);
        }
    }
}

