use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Branch {
    pub index: i32,
    pub time: f64,
    //inbounds: RefCell<Weak<Node>>,
    pub outbounds: RefCell<Rc<Node>>,
}

#[derive(Debug, Default)]
pub struct Node {
    pub index: i32, 
    pub label: String,
    //parent: RefCell<Weak<Branch>>,
    pub children: RefCell<Vec<Rc<Branch>>>,
}
