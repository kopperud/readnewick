use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::rc::Rc;

//use std::{cell::RefCell, rc::Rc};

//pub type NodeRef = Rc<RefCell<Node>>;
//pub type BranchRef = Rc<RefCell<Branch>>;

#[derive(Default)]
pub struct Branch {
    inbounds: Option<Rc<Node>>,
    outbounds: Option<Rc<Node>>,
}

#[derive(Default)]
pub struct Node {
    inbounds: Option<Rc<Branch>>,
    left: Option<Rc<Branch>>,
    right: Option<Rc<Branch>>,
}

impl Node{
    pub fn set_left(&self, branch: Rc<Branch>) {
        self.left = Some(branch);
        //branch.inbounds = Some(&self);
    }

    pub fn set_right(&self, branch: Rc<Branch>) {
       self.right = Some(branch);
    }
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

            let is_special = special_tokens.contains(&c);
            token.push(c);

            if is_special{
                tokens.push(token);
                token = "".to_string();
            }else{
                let next = iter.peek().expect("reason");
                if special_tokens.contains(next){
                    tokens.push(token);
                    token = "".to_string();
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

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("primates.tre").expect("should have been able to readfile");

    let stripped_contents = stripcomments(&contents); 


    //println!("With text: \n {stripped_contents}");

    let s = "(((A:0.5,B:0.5):0.5):1.0,C:1.5);";
    //println!("{}", v[0]);
    let v = tokenize(&s);
    //let v = tokenize(&stripped_contents);

    for i in v.iter(){
        let isp = i == ",";
        println!("{i} \t, is comma = {}", isp);
    }

    let node = Rc::new(Node::default());
    let branch_left = Rc::new(Branch::default());
    let branch_right = Rc::new(Branch::default());

    //node.left = Some(branch_left);
    node.set_left(Rc::clone(&branch_left));
    //
    //let node_ref = &node;
    //let branch_left_ref = &branch_left;

    //node.left = Some(branch_left_ref);
    //node.right = Some(branch_right);

    //branch_left.inbounds = Some(node);


    /* let mut chars2 = s.chars();
    let mut iter2 = chars2.peekable();


    let u_item = iter2.position(|x| x == ')');
    let item = u_item.unwrap(); */

    // println!("{}", item);
    
}
