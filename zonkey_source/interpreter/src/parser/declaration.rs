use crate::parser::value::ValueType;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub id: usize,
    pub parameters: Vec<(ValueType, String)>,
    pub return_data_type: Option<ValueType>,
}

#[derive(Debug)]
pub struct ClassDeclaration {
    pub properties: FxHashMap<String, ValueType>,
}
