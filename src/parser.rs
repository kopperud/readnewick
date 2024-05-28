use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::*;
use crate::utils::*;
use crate::tokenizer::*;

pub fn parse_tree(contents: String) -> Rc<Node> {
    let newickstring = find_newick_string(contents);

    let stripped_contents = stripcomments(&newickstring); 
    let tokens = tokenize(&stripped_contents);
    let root = parse_newick(&tokens);

    return root
}


pub fn parse_newick(tokens: &[String]) -> Rc<Node> {
    let node = Rc::new(Node {
        index: 1,
        label: "".to_string(),
        children: RefCell::new(vec![]),
    });
    
    // strip semicolon
    let n_minus_one = tokens.len() - 1;
    let slice = &tokens[1..n_minus_one];
    let sides = partition(&slice);

    for side in sides{
        if !side.is_empty(){
            if side.len() == 1{
                terminaledge(side, &node);
            }else{
                internaledge(side, &node); 
            }
        }
    }

    return node
}

fn terminaledge(tokens: &[String], parent_node: &Rc<Node>){
    //println!("tokens for terminal: \t {:?}", tokens);
    assert!(tokens.len() == 1);

    let end_token = tokens.last().expect("reason");
    let l = parse_brlen(end_token);
    let species_name = parse_speciesname(end_token);

    let node = Rc::new(Node {
        index: 1,
        label: species_name.to_string(),
        children: RefCell::new(vec![]),
    });
    let branch1 = Rc::new(Branch {
        index: 1,
        time: l,
        outbounds: RefCell::new(Rc::clone(&node)),
    });
    parent_node.children.borrow_mut().push(Rc::clone(&branch1));
    
}

fn internaledge(tokens: &[String], parent_node: &Rc<Node>) {
    // strip parentheses
    //println!("tokens for internaledge: \t {:?}", &tokens);
    let l = parse_brlen(tokens.last().expect("reason"));

    let n_minus_one = tokens.len() - 1;
    let slice = &tokens[1..n_minus_one];

    // add a new internal node and branch
    let node = Rc::new(Node {
        index: 1,
        label: "".to_string(),
        children: RefCell::new(vec![]),
    });
    let branch1 = Rc::new(Branch {
        index: 1,
        time: l,
        outbounds: RefCell::new(Rc::clone(&node)),
    });
    parent_node.children.borrow_mut().push(Rc::clone(&branch1));

    let sides = partition(&slice);

    for side in sides{
        if !side.is_empty(){
            if side.len() == 1{
                terminaledge(side, &node);
            }else{
                internaledge(side, &node); 
            }
        }
    }
}

fn find_separators(tokens: &[String]) -> Vec<usize> {
    let mut ps = 0;

    let n_tokens = tokens.len();
    let mut comma_positions: Vec<usize> = Vec::new();

    for i in 0..n_tokens {
        let token = &tokens[i];
        if token == "(" {
            ps += 1;
        }else if token == ")" {
            ps -= 1;
        }

        if (token == ",") & (ps == 0){
            comma_positions.push(i);
        }
    }

    if comma_positions.is_empty(){
        println!("tokens before crash: {:?}", tokens);
        panic!("crash and burn");
    }

    return comma_positions
}

fn partition(tokens: &[String]) -> Vec<&[String]> {
    let n_tokens = tokens.len();

    let comma_positions = find_separators(&tokens); 
    let mut start: usize = 0;

    

    let mut sides: Vec<&[String]> = Vec::new(); 
    for cp in comma_positions{
         let side = &tokens
             .get(start..cp)
             .unwrap(); 
        start = cp + 1;
         
        sides.push(side);
    }
    let side = &tokens
        .get(start..(n_tokens-1))
        .unwrap();
    sides.push(side);

    return sides
}

fn parse_brlen(token: &str) -> f64 {
    let colon_pos = token.find(':').unwrap();

    let trailing = &token[(colon_pos+1)..];
    let branch_length: f64 = trailing.parse().unwrap();
    return branch_length
}

fn parse_speciesname(token: &str) -> &str {
    let colon_pos = token.find(':').unwrap();

    let species_name = &token[..colon_pos];
    return species_name
}
