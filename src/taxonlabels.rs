use crate::tree::*;

pub fn taxon_labels(
    node: &Box<Node>
    ) -> Vec<String> 
{
    let mut taxa: Vec<String> = vec![];

    taxon_labels_po(&mut taxa, node);
    taxa
}

fn taxon_labels_po(
    taxa: &mut Vec<String>, 
    node: &Box<Node>
    )
{
    let children = &node.children;

    if children.is_empty(){
        taxa.push(node.label.clone());
    }else{
        for child in children.iter(){
            taxon_labels_po(taxa, child);
        }
    }
}

