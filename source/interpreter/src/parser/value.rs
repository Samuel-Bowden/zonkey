use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(usize),
    Float(usize),
    String(usize),
    Boolean(usize),
    Object(Rc<String>, usize),
}

impl Value {
    pub fn to_value_type(self) -> ValueType {
        match self {
            Value::Integer(_) => ValueType::Integer,
            Value::Float(_) => ValueType::Float,
            Value::String(_) => ValueType::String,
            Value::Boolean(_) => ValueType::Boolean,
            Value::Object(class, _) => ValueType::Class(class),
        }
    }
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
