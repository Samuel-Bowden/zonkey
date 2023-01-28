use crate::value_type::ValueType;

#[derive(Clone)]
pub struct FunctionDeclaration {
    pub id: usize,
    pub parameters: Vec<(ValueType, String)>,
    pub return_data_type: Option<ValueType>,
}
