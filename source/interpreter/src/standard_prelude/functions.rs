use crate::parser::{
    declaration::{CallableDeclaration, CallableType},
    value::ValueType,
};
use rustc_hash::FxHashMap;
use std::rc::Rc;

pub fn new() -> FxHashMap<Rc<String>, CallableDeclaration> {
    let mut functions = FxHashMap::default();

    functions.insert(
        Rc::new("print".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Any],
            return_type: None,
        },
    );

    functions.insert(
        Rc::new("println".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Any],
            return_type: None,
        },
    );

    functions.insert(
        Rc::new("prompt".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::String),
        },
    );

    functions.insert(
        Rc::new("sleep".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Integer],
            return_type: None,
        },
    );

    functions.insert(
        Rc::new("close_tab".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: None,
        },
    );

    functions.insert(
        Rc::new("set_page".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Class(Rc::new("Page".into()))],
            return_type: None,
        },
    );

    functions.insert(
        Rc::new("wait_for_event".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Boolean),
        },
    );

    functions.insert(
        Rc::new("Button".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::new("Button".to_string()))),
        },
    );

    functions.insert(
        Rc::new("Page".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Class(Rc::new("Page".to_string()))),
        },
    );

    functions.insert(
        Rc::new("Text".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::new("Text".to_string()))),
        },
    );

    functions.insert(
        Rc::new("Hyperlink".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String, ValueType::String],
            return_type: Some(ValueType::Class(Rc::new("Hyperlink".to_string()))),
        },
    );

    functions.insert(
        Rc::new("Input".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::new("Input".to_string()))),
        },
    );

    functions.insert(
        Rc::new("Row".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Class(Rc::new("Row".to_string()))),
        },
    );

    functions.insert(
        Rc::new("Column".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Class(Rc::new("Column".to_string()))),
        },
    );

    functions.insert(
        Rc::new("Image".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::new("Image".to_string()))),
        },
    );

    functions
}
