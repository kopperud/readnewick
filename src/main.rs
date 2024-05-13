use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

//use std::{cell::RefCell, rc::Rc};

//pub type NodeRef = Rc<RefCell<Node>>;
//pub type BranchRef = Rc<RefCell<Branch>>;

#[derive(Debug, Default)]
pub struct Branch {
    value: i32,
    nodes: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug, Default)]
pub struct Node {
    value: i32, 
    branches: RefCell<Vec<Weak<Branch>>>,
}
/*
impl Node{
    pub fn set_left(&self, branch: Rc<Branch>) {
        self.left = Some(branch);
        //branch.inbounds = Some(&self);
    }

    pub fn set_right(&self, branch: Rc<Branch>) {
       self.right = Some(branch);
    }
}
*/


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
   
    let leaf1 = Rc::new(Node {
        value: 1,
        branches: RefCell::new(vec![Weak::new()]),
    });

    let leaf2 = Rc::new(Node {
        value: 2,
        branches: RefCell::new(vec![Weak::new()]),
    });

    let internal_node = Rc::new(Node {
        value: 3,
        branches: RefCell::new(vec![]),
    });

    let branch1 = Rc::new(Branch {
        value: 1,
        nodes: RefCell::new(vec![]),
    });

    //push leaf and internal node to branch1
    branch1.nodes.borrow_mut().push(Rc::clone(&leaf1));
    branch1.nodes.borrow_mut().push(Rc::clone(&internal_node));

    let branch2 = Rc::new(Branch {
        value: 2,
        nodes: RefCell::new(vec![]),
    });

    //push leaf and internal node to branch2
    branch2.nodes.borrow_mut().push(Rc::clone(&leaf2));
    branch2.nodes.borrow_mut().push(Rc::clone(&internal_node));

    //push branch to the leaf nodes
    leaf1.branches.borrow_mut().push(Rc::downgrade(&branch1));
    leaf2.branches.borrow_mut().push(Rc::downgrade(&branch2));

    //push branches to the internal node 
    internal_node.branches.borrow_mut().push(Rc::downgrade(&branch1));
    internal_node.branches.borrow_mut().push(Rc::downgrade(&branch2));

    let v: Vec<i32> = Vec::new();

    println!("{:?}", internal_node)
}
