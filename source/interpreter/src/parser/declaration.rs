use crate::{expr::Expr, parser::value::ValueType};
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
    pub class_type: ClassType,
    pub methods: FxHashMap<Rc<String>, Rc<CallableDeclaration>>,
}

#[derive(Debug)]
pub enum ClassType {
    Native,
    Zonkey(Rc<Vec<Expr>>),
}
