use crate::parser::value::ValueType;
use rustc_hash::FxHashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CallableDeclaration {
    pub callable_type: CallableType,
    pub parameters: Vec<ValueType>,
    pub return_type: Option<ValueType>,
}

#[derive(Debug, Clone)]
pub enum CallableType {
    Native,
    Zonkey(usize),
}

#[derive(Debug)]
pub struct ClassDeclaration {
    pub properties: FxHashMap<Rc<String>, ValueType>,
    pub methods: FxHashMap<Rc<String>, Rc<CallableDeclaration>>,
}
