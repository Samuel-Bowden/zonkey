use std::rc::Rc;

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
