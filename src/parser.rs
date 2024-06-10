use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::tree::*;
use crate::utils::*;
use crate::tokenizer::*;

pub fn parse_tree(contents: String) -> Rc<Node> {
    let newickstring = find_newick_string(contents);

    let stripped_contents = stripcomments(&newickstring); 
    let tokens = tokenize(&stripped_contents);
    //let root = parse_newick(tokens);
    //root
    parse_newick(tokens)
}


pub fn parse_newick(tokens: VecDeque<&str>) -> Rc<Node> {
    let node = Rc::new(Node {
        index: 1,
        label: "".to_string(),
        children: RefCell::new(vec![]),
    });
    
    // strip semicolon
    let mut slice = tokens.clone();
    //slice.remove(slice.len()-1);
    //slice.remove(0);
    slice.pop_front();
    slice.pop_back();

    let sides = partition(slice);

    for side in sides{
        if !side.is_empty(){
            if side.len() == 1{
                terminaledge(side, &node);
            }else{
                internaledge(side, &node); 
            }
        }
    }

    node
}

fn terminaledge(tokens: VecDeque<&str>, parent_node: &Rc<Node>){
    //println!("tokens for terminal: \t {:?}", tokens);
    assert!(tokens.len() == 1);

    let end_token = *tokens.back().expect("reason");
    //let l = parse_brlen(end_token);
    let species_name = parse_speciesname(end_token);

    let node = Rc::new(Node {
        index: 1,
        label: species_name.to_string(),
        children: RefCell::new(vec![]),
    });
    /* let branch1 = Rc::new(Branch {
        index: 1,
        time: l,
        outbounds: RefCell::new(Rc::clone(&node)),
    });*/
    parent_node.children.borrow_mut().push(Rc::clone(&node));
    
}

fn internaledge(tokens: VecDeque<&str>, parent_node: &Rc<Node>) {
    // strip parentheses
    //println!("tokens for internaledge: \t {:?}", &tokens);
    //let l = parse_brlen(tokens.last().expect("reason"));

    //let slice = &tokens[1..n_minus_one];
    let mut slice = tokens.clone();
    slice.remove(slice.len()-1);
    slice.remove(0);

    let internal_label: String = "".to_string();

    // add a new internal node and branch
    let node = Rc::new(Node {
        index: 1,
        label: internal_label,
        children: RefCell::new(vec![]),
    });

    parent_node.children.borrow_mut().push(Rc::clone(&node));

    let sides = partition(slice);

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

fn find_separators(tokens: VecDeque<&str>) -> Vec<usize> {
    let mut ps = 0;

    let n_tokens = tokens.len();
    let mut comma_positions: Vec<usize> = Vec::new();

    //for i in 0..n_tokens {
    //let token = tokens[i];
    for (i, token) in tokens.iter().enumerate().take(n_tokens){
        if *token == "(" {
            ps += 1;
        }else if *token == ")" {
            ps -= 1;
        }

        if (*token == ",") & (ps == 0){
            comma_positions.push(i);
        }
    }

    if comma_positions.is_empty(){
        println!("tokens before crash: {:?}", tokens);
        panic!("crash and burn");
    }

    comma_positions
}

fn partition(tokens: VecDeque<&str>) -> Vec<VecDeque<&str>> {
    let n_tokens = tokens.len();

    let comma_positions = find_separators(tokens.clone()); 
    let mut start: usize = 0;
    

    let mut sides: Vec<VecDeque<&str>> = Vec::new(); 
    

    for cp in comma_positions{
        let mut side: VecDeque<&str> = VecDeque::new();
        for token in tokens.range(start..cp){
            side.push_back(*token);
        }

        start = cp + 1;
         
        sides.push(side);
    }

    let mut side: VecDeque<&str> = VecDeque::new();
    for token in tokens.range(start..(n_tokens-1)){
        side.push_back(*token);
    }
    sides.push(side);

    sides
}

/*
fn parse_brlen(token: &str) -> f64 {
    let colon_pos = token.find(':').unwrap();

    let trailing = &token[(colon_pos+1)..];
    let branch_length: f64 = trailing.parse().unwrap();
    branch_length
}
*/

fn parse_speciesname(token: &str) -> &str {
    let colon_pos = token.find(':').expect("expected to find a token starting with a colon (\":\")");

    //let species_name = &token[..colon_pos];
    //species_name
    &token[..colon_pos]
}


