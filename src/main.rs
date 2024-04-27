use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::{cell,RefCell, rc::Rc};

struct Branch {
    inbounds: 
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
    //let v = tokenize(&stripped_contents);
    //println!("{}", v[0]);
    //let v = tokenize(&s);
    let v = tokenize(&stripped_contents);

    for i in v.iter(){
        let isp = i == ",";
        println!("{i} \t, is comma = {}", isp);
    }

    /* let mut chars2 = s.chars();
    let mut iter2 = chars2.peekable();


    let u_item = iter2.position(|x| x == ')');
    let item = u_item.unwrap(); */

    // println!("{}", item);
    
}
