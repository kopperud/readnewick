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
    //inbounds: RefCell<Weak<Node>>,
    outbounds: RefCell<Rc<Node>>,
}

#[derive(Debug, Default)]
pub struct Node {
    index: i32, 
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
        //parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let leaf1 = Rc::new(Node {
        index: 1,
        //parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let leaf2 = Rc::new(Node {
        index: 2,
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

    println!("{:?}", leaf1.children.borrow().len());
    return root_node
}

fn parse_newick() -> Rc<Node> {
    let node = Rc::new(Node {
        index: 1,
        children: RefCell::new(vec![]),
    });
    return node
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

    panic!("crash and burn");
}

fn left_right_tokens(tokens: &[String]) -> (&[String], &[String]) {
    let ps = find_comma(&tokens);

    //let n_tokens = tokens.len();
    //let n_minus_one = n_tokens - 1;

    //let slice = &tokens[1..n_minus_two];

    let left = &tokens[1..ps];
    let right = &tokens[(ps+1)..];

    return (left, right)
}

fn parse_brlen(token: &str) -> f64 {
    let colon_pos = token.find(':').unwrap();

    let trailing = &token[(colon_pos+1)..];
    let branch_length: f64 = trailing.parse().unwrap();
    return branch_length
}

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("primates.tre").expect("should have been able to readfile");

    let stripped_contents = stripcomments(&contents); 


    //println!("With text: \n {stripped_contents}");

    let s = "(((A:0.5,B:0.5):0.5):1.0,C:1.5);";
    //println!("{}", tokens[0]);
    let tokens = tokenize(&s);
    //let tokens = tokenize(&stripped_contents);

    for token in tokens.iter(){
        let isp = token == ",";
        println!("{token} \t, is comma = {}", isp);
    }
  
    let root_node = dummy();

    println!("{:?}", root_node);
    println!("{:?}", root_node.children.borrow()[0].outbounds.borrow());

    let n_tokens = tokens.len();
    let n_minus_two = n_tokens - 2;

    let slice = &tokens[1..n_minus_two];

    println!("{:?}", &slice);
    let ps = find_comma(&slice);

    println!("comma position: \t {ps}"); 

    //let left = &slice[1..ps];
    //let right = &slice[(ps+1)..];
    //
    let (left, right) = left_right_tokens(&slice);

    println!("left: \t {:?}", &left);
    println!("right: \t {:?}", &right);

    let last_token = &left.last().unwrap();
    println!("last token: \t {:?}", &last_token);
    println!("position of colon: \t {:?}", last_token.find(':').unwrap());
    println!("position of colon in \"HomoSapiens:0.123\": \t {:?}", "HomoSapiens:0.123".find(':').unwrap());

    let fake_token = "Homo_sapiens:15.123";
    println!("fake token: \t {}", &fake_token);
    println!("with branch length: \t {}", parse_brlen(fake_token));
    let v: Vec<i32> = Vec::new();

}


