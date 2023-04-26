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
            parameters: vec![ValueType::Printable],
            return_type: None,
        },
    );

    functions.insert(
        Rc::new("println".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Printable],
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
        Rc::new("args".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Class(Rc::new("[String]".into()))),
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
        Rc::new("install_application".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![
                ValueType::Class(Rc::new("[String]".into())),
                ValueType::Boolean,
            ],
            return_type: None,
        },
    );

    functions.insert(
        Rc::new("remove_application".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: None,
        },
    );

    functions.insert(
        Rc::new("installed_applications".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Class(Rc::new("[String]".into()))),
        },
    );

    functions.insert(
        Rc::new("read_string".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::String),
        },
    );

    functions.insert(
        Rc::new("write_string".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String, ValueType::String],
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
        Rc::new("open_link".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![
                ValueType::String,
                ValueType::Class(Rc::new("[String]".into())),
            ],
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
        Rc::new("integer_to_string".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Integer],
            return_type: Some(ValueType::String),
        },
    );

    functions.insert(
        Rc::new("float_to_string".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float],
            return_type: Some(ValueType::String),
        },
    );

    functions.insert(
        Rc::new("string_to_integer".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Integer),
        },
    );

    functions.insert(
        Rc::new("string_to_float".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Float),
        },
    );

    functions.insert(
        Rc::new("integer_to_float".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Integer],
            return_type: Some(ValueType::Float),
        },
    );

    functions.insert(
        Rc::new("float_to_integer".to_string()),
        CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float],
            return_type: Some(ValueType::Integer),
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
