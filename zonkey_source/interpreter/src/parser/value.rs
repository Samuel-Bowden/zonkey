use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(usize),
    Float(usize),
    String(usize),
    Boolean(usize),
    Object(Rc<String>, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Integer,
    Float,
    String,
    Boolean,
    Any,
    Element,
    Class(Rc<String>),
}
