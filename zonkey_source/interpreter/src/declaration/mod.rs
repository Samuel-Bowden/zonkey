use rustc_hash::FxHashMap;
use crate::{value_type::ValueType, return_type::ReturnType};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub id: usize,
    pub parameters: Vec<(ValueType, String)>,
    pub return_data_type: ReturnType,
}

#[derive(Debug)]
pub struct ClassDeclaration {
    pub properties: FxHashMap<String, ValueType>,
}
