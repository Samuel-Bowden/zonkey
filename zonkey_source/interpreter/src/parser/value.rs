use indexmap::IndexMap;

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
    Class(String),
}

#[derive(Debug, Clone)]
pub struct Object {
    pub class_declaration: String,
    pub properties: IndexMap<String, Value>,
}
