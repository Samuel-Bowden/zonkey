use crate::{value_type::ValueType, return_type::ReturnType};

#[derive(Debug, Clone)]
pub struct CallableDeclaration {
    pub id: usize,
    pub parameters: Vec<(ValueType, String)>,
    pub return_data_type: ReturnType,
    pub declaration_type: DeclarationType,
}

#[derive(Debug, Clone)]
pub enum DeclarationType {
    Function,
    Class(Vec<(ValueType, String)>),
}
