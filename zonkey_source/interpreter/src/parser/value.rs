use rustc_hash::FxHashMap;
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
    Class(Rc<String>),
}

#[derive(Debug, Clone)]
pub struct Object {
    pub properties: FxHashMap<Rc<String>, Value>,
    pub objects: FxHashMap<usize, Rc<Object>>,
    pub integer_next_id: usize,
    pub float_next_id: usize,
    pub string_next_id: usize,
    pub boolean_next_id: usize,
    pub object_next_id: usize,
}
