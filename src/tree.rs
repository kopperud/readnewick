use std::rc::Rc;
use std::cell::RefCell;

/*
#[derive(Debug, Default)]
pub struct Branch {
    pub index: i32,
    pub time: f64,
    pub outbounds: RefCell<Rc<Node>>,
}
*/

#[derive(Debug, Default)]
pub struct Node {
    pub index: i32, 
    pub label: String,
    pub children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug, Default)]
pub struct Node2 {
    pub index: i32, 
    pub label: RefCell<String>,
    pub children: RefCell<Vec<Rc<Node2>>>,
}
