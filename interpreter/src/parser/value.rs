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
    Printable,
    Element,
    Generic,
    Class(Rc<String>),
}

pub fn print_type<'a>(value_type: &'a Option<ValueType>) -> &'a str {
    match value_type {
        Some(ValueType::Integer) => "Integer",
        Some(ValueType::Float) => "Float",
        Some(ValueType::String) => "String",
        Some(ValueType::Boolean) => "Boolean",
        Some(ValueType::Printable) => "Printable",
        Some(ValueType::Element) => "Element",
        Some(ValueType::Generic) => "Generic",
        Some(ValueType::Class(name)) => &name,
        None => "none",
    }
}
