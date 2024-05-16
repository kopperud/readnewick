use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::*;

pub fn parse_newick(tokens: &[String]) -> Rc<Node> {
    let node = Rc::new(Node {
        index: 1,
        label: "".to_string(),
        children: RefCell::new(vec![]),
    });
    // strip semicolon
    let n_minus_two = tokens.len() - 2;
    let slice = &tokens[1..n_minus_two];
    let (left, right) = partition(&slice);

    if !left.is_empty(){
        if left.len() == 1{
            terminaledge(left, &node);
        }else{
            internaledge(left, &node); 
        }
    } 
    
    if !right.is_empty(){
        if right.len() == 1{
            terminaledge(right, &node);
        }else{
            internaledge(right, &node); 
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
    
    let (left, right) = partition(&slice);
   
    if !left.is_empty(){
        if left.len() == 1{
            terminaledge(left, &node);
        }else{
            internaledge(left, &node); 
        }
    } 
    
    if !right.is_empty(){
        if right.len() == 1{
            terminaledge(right, &node);
        }else{
            internaledge(right, &node); 
        }
    }
}

fn find_comma(tokens: &[String]) -> usize {
    let mut ps = 0;

    let n_tokens = tokens.len();

    for i in 0..n_tokens {
        let token = &tokens[i];
        if token == "(" {
            ps += 1;
        }else if token == ")" {
            ps -= 1;
        }

        if (token == ",") & (ps == 0){
            return i
        }
    }
    println!("tokens before crash: {:?}", tokens);
    panic!("crash and burn");
}

fn partition(tokens: &[String]) -> (&[String], &[String]) {
    let ps = find_comma(&tokens);
    let n_tokens = tokens.len();

    let left = &tokens[0..ps];
    let right = &tokens[(ps+1)..(n_tokens-1)];

    return (left, right)
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
