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
    pub label: String,
    pub children: Vec<Box<Node>>,
}
