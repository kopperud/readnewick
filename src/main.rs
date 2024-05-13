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
    index: i32,
    time: f64,
    inbounds: RefCell<Weak<Node>>,
    outbounds: RefCell<Rc<Node>>,
}

#[derive(Debug, Default)]
pub struct Node {
    index: i32, 
    parent: RefCell<Weak<Branch>>,
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

fn dummy() -> Rc<Node> {
    let root_node = Rc::new(Node {
        index: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let leaf1 = Rc::new(Node {
        index: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let leaf2 = Rc::new(Node {
        index: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch1 = Rc::new(Branch {
        index: 1,
        time: 1.0,
        inbounds: RefCell::new(Rc::downgrade(&root_node)),
        outbounds: RefCell::new(Rc::clone(&leaf1)),
    });
    root_node.children.borrow_mut().push(Rc::clone(&branch1));
    *leaf1.parent.borrow_mut() = Rc::downgrade(&branch1);

    let branch2 = Rc::new(Branch {
        index: 2,
        time: 1.0,
        inbounds: RefCell::new(Rc::downgrade(&root_node)),
        outbounds: RefCell::new(Rc::clone(&leaf2)),
    });
    root_node.children.borrow_mut().push(Rc::clone(&branch2));
    *leaf2.parent.borrow_mut() = Rc::downgrade(&branch2);

    println!("{:?}", leaf1.children.borrow().len());
    return root_node
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
  
    let root_node = dummy();

    let v: Vec<i32> = Vec::new();

}


