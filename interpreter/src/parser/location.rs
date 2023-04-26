use super::value::ValueType;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Location {
    Integer(usize),
    Float(usize),
    String(usize),
    Boolean(usize),
    Object(Rc<String>, usize),
}

impl Location {
    pub fn to_value_type(self) -> ValueType {
        match self {
            Location::Integer(_) => ValueType::Integer,
            Location::Float(_) => ValueType::Float,
            Location::String(_) => ValueType::String,
            Location::Boolean(_) => ValueType::Boolean,
            Location::Object(class, _) => ValueType::Class(class),
        }
    }
}
