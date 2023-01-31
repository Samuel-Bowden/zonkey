use crate::{return_type::ReturnType, value_type::ValueType};

#[derive(Clone)]
pub struct FunctionDeclaration {
    pub id: usize,
    pub parameters: Vec<(ValueType, String)>,
    pub return_data_type: ReturnType,
}
