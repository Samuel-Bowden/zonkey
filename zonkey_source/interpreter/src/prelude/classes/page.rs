use crate::parser::{
    declaration::{CallableDeclaration, CallableType, ClassDeclaration},
    value::ValueType,
};
use rustc_hash::FxHashMap;
use std::rc::Rc;

pub fn new(_: Rc<String>) -> ClassDeclaration {
    let properties = FxHashMap::default();

    let mut methods = FxHashMap::default();

    methods.insert(
        "add".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Element],
            return_type: None,
        }),
    );

    ClassDeclaration {
        properties,
        methods,
        property_default_expressions: vec![],
    }
}
