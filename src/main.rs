use std::fs;
use regex::Regex;

fn tokenize(s: &str) -> Vec<String> {
    //let v: Vec<str> = Vec::new();
    let mut v: Vec<String> = Vec::new();

    v.push("hello".to_string());
    return v;
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


    println!("With text: \n {stripped_contents}");

    let v = tokenize(&stripped_contents);
    println!("{}", v[0]);


    let mut count = 0u32;

    let mut chars = v[0].chars();
    let mut iter = chars.peekable();

    while let Some(_) = iter.next(){
        if let Some(c) = iter.peek(){
            //let item = chars.nth(0).unwrap();
            println!("{}", c);
            println!("is a parenthesis: {}", c == &'(');
            count += 1;
        }
    }

    let s = "this (inside parenthesis) is a sentence".to_string();
    let mut chars2 = s.chars();
    let mut iter2 = chars2.peekable();

    let mut inside = String::new();

    let u_item = iter2.position(|x| x == ')');
    let item = u_item.unwrap();

    println!("{}", item);
    
}
