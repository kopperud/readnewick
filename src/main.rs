use std::fs;
use std::env;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashSet;
use microbench::{self, Options};
use regex::Regex;

//use std::{cell::RefCell, rc::Rc};

//pub type NodeRef = Rc<RefCell<Node>>;
//pub type BranchRef = Rc<RefCell<Branch>>;

#[derive(Debug, Default)]
pub struct Branch {
    index: i32,
    time: f64,
    //inbounds: RefCell<Weak<Node>>,
    outbounds: RefCell<Rc<Node>>,
}

#[derive(Debug, Default)]
pub struct Node {
    index: i32, 
    label: String,
    //parent: RefCell<Weak<Branch>>,
    children: RefCell<Vec<Rc<Branch>>>,
}


fn tokenize(s: &str) -> Vec<String> {
    //let tokens: Vec<str> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();

    let mut special_tokens = HashSet::new();
    special_tokens.insert('(');
    special_tokens.insert(')');
    special_tokens.insert(';');
    special_tokens.insert(',');
    special_tokens.insert('[');
    special_tokens.insert(']');


    let chars = s.chars();
    let mut iter = chars.peekable();
    let mut token = "".to_string();


    while let Some(_) = iter.peek(){
        if let Some(c) = iter.next(){

            println!("{:?}", &c);
            if c != 0xA as char {

                let is_special = special_tokens.contains(&c);
                token.push(c);

                if is_special{
                    tokens.push(token);
                    token = "".to_string();
                }else{
                    let next = iter.peek().expect("should have been able to peek to next char");
                    if special_tokens.contains(next){
                        tokens.push(token);
                        token = "".to_string();
                    }
                }
            }
        }
    }

    let node = Node::default();

    return tokens;
}

fn stripcomments(contents: &str) -> String {
    let re = Regex::new(r"\[.*?\]").unwrap();
    let stripped_contents = re.replace_all(&contents, "");

    return stripped_contents.to_string();
}

fn dummy() -> Rc<Node> {
    let root_node = Rc::new(Node {
        index: 3,
        label: "".to_string(),
        //parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let leaf1 = Rc::new(Node {
        index: 1,
        label: "Homo sapiens".to_string(),
        //parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let leaf2 = Rc::new(Node {
        index: 2,
        label: "Homo erectus".to_string(),
        //parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch1 = Rc::new(Branch {
        index: 1,
        time: 1.0,
        //inbounds: RefCell::new(Rc::downgrade(&root_node)),
        outbounds: RefCell::new(Rc::clone(&leaf1)),
    });
    root_node.children.borrow_mut().push(Rc::clone(&branch1));
    //*leaf1.parent.borrow_mut() = Rc::downgrade(&branch1);

    let branch2 = Rc::new(Branch {
        index: 2,
        time: 1.0,
        //inbounds: RefCell::new(Rc::downgrade(&root_node)),
        outbounds: RefCell::new(Rc::clone(&leaf2)),
    });
    root_node.children.borrow_mut().push(Rc::clone(&branch2));
    //*leaf2.parent.borrow_mut() = Rc::downgrade(&branch2);

    return root_node
}

fn parse_newick(tokens: &[String]) -> Rc<Node> {
    let node = Rc::new(Node {
        index: 1,
        label: "".to_string(),
        children: RefCell::new(vec![]),
    });
    // strip semicolon
    let n_minus_two = tokens.len() - 2;
    let slice = &tokens[1..n_minus_two];
    let (left, right) = partition(&slice);

    println!("right from root: \t {:?}", &right);
    
    if !left.is_empty(){
        //if left.last().expect("reason").starts_with(':'){
        if left.len() == 1{
            terminaledge(left, &node);
        }else{
            internaledge(left, &node); 
        }
    } 
    
    if !right.is_empty(){
        //if right.last().expect("reason").starts_with(':'){
        if right.len() == 1{
            terminaledge(right, &node);
        }else{
            internaledge(right, &node); 
        }
    }

    return node
}

fn terminaledge(tokens: &[String], parent_node: &Rc<Node>){
    println!("tokens for terminal: \t {:?}", tokens);
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
    //println!("tokens: \t {:?}", &tokens);
    let l = parse_brlen(tokens.last().expect("reason"));

    let n_minus_one = tokens.len() - 1;
    //println!("before slice: {:?}", &tokens);
    let slice = &tokens[1..n_minus_one];

    //let slice = &slice0[..(slice0.len()-1)];
    //println!("after slice: {:?}", &slice);

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
   
    //println!("left: \t {:?}", &left);
    //println!("right: \t {:?}", &right);
    if !left.is_empty(){
        //if left.last().expect("reason").starts_with(':'){
        if left.len() == 1{
            terminaledge(left, &node);
        }else{
            internaledge(left, &node); 
        }
    } 
    
    if !right.is_empty(){
        //if right.last().expect("reason").starts_with(':'){
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
    println!("token for parse brlen: \t {:?}", token);
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

fn taxon_labels(root: &Rc<Node>) -> Vec<String> {
    let mut taxa: Vec<String> = vec![];

    for child_branch in root.children.borrow().iter(){
        taxon_labels_po(&mut taxa, &child_branch.outbounds.borrow());
    }
    return taxa
}

fn taxon_labels_po(taxa: &mut Vec<String>, node: &Rc<Node>){
    let children = node.children.borrow();

    if children.is_empty(){
        taxa.push(node.label.clone());
    }else{
        for child_branch in children.iter(){
            //child_node = child_branch.outbounds.borrow();
            taxon_labels_po(taxa, &child_branch.outbounds.borrow());
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
    //println!("{}", tokens[0]);
    //let tokens = tokenize(&s);
    let tokens = tokenize(&stripped_contents);

    for token in tokens.iter(){
        let isp = token == ",";
        println!("{token} \t, is comma = {}", isp);
    }
  

    //let left = &slice[1..ps];
    //let right = &slice[(ps+1)..];



    let root = parse_newick(&tokens);
    //println!("root tree: {:?}", &root);
    println!("taxon labels: \t {:?}", taxon_labels(&root));

    let options = Options::default();
    microbench::bench(&options, "collect leaf labels", || taxon_labels(&root));
}


