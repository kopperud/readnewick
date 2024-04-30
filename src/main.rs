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
    inbounds: Option<RefCell<Weak<Node>>>,
    outbounds: Option<RefCell<Rc<Node>>>,
}

#[derive(Debug, Default)]
pub struct Node {
    inbounds: Option<RefCell<Weak<Branch>>>,
    left: Option<RefCell<Rc<Branch>>>,
    right: Option<RefCell<Rc<Branch>>>,
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

    let node = Node {
            inbounds: Some(RefCell::new(Weak::new())),
            //left: RefCell::new(Rc::new(Branch)),
            //right: RefCell::new(Rc::new(Branch)),
            left: None,
            right: None,
        };

    let branch = Branch {
        inbounds: Some(RefCell::new(Weak::new())),
        outbounds: None,
    }


/*
    let branch = Rc::new(
            Branch {
               inbounds: RefCell::new(Weak::new()),
               outbounds: RefCell::new(Rc::new()),
            }
        );
    */

    //let node = Rc::new(Node::default());
    //let branch_left = Rc::new(Branch::default());

//    branch_left.inbounds = RefCell::new(Rc::clone(&node))
    
    //branch_left.inbounds = Rc::Weak(Rc::downgrade(&node));


    //let branch_right = Rc::new(Branch::default());

    //node.left = Some(branch_left);
    //node.set_left(Rc::clone(&branch_left));
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
