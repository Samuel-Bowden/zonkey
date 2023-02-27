use crate::value_type::ValueType;

#[derive(Debug)]
pub struct Class {
    pub properties: Vec<(ValueType, usize)>,
}
