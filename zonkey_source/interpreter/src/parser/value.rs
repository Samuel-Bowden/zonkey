use indexmap::IndexMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(usize),
    Float(usize),
    String(usize),
    Boolean(usize),
    Object(Object),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Integer,
    Float,
    String,
    Boolean,
    Any,
    Class(Rc<String>),
}

#[derive(Debug, Clone)]
pub struct Object {
    pub class_declaration: Rc<String>,
    pub properties: IndexMap<Rc<String>, Value>,
}
