use crate::{
    expr::{Expr, FloatExpr, IntegerExpr, StringExpr},
    parser::{
        declaration::{CallableDeclaration, CallableType, ClassDeclaration},
        value::{Value, ValueType},
    },
};
use rustc_hash::FxHashMap;
use std::rc::Rc;

pub fn new(text: Rc<String>) -> ClassDeclaration {
    let mut properties = FxHashMap::default();

    properties.insert("text".to_string().into(), Value::String(0));
    properties.insert("id".to_string().into(), Value::Integer(0));
    properties.insert("size".to_string().into(), Value::Float(0));
    properties.insert("red".to_string().into(), Value::Float(1));
    properties.insert("green".to_string().into(), Value::Float(2));
    properties.insert("blue".to_string().into(), Value::Float(3));

    let mut methods = FxHashMap::default();

    methods.insert(
        "set_value".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::clone(&text))),
        }),
    );

    methods.insert(
        "set_size".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float],
            return_type: Some(ValueType::Class(Rc::clone(&text))),
        }),
    );

    methods.insert(
        "set_colour".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float, ValueType::Float, ValueType::Float],
            return_type: Some(ValueType::Class(Rc::clone(&text))),
        }),
    );

    let property_default_expressions = vec![
        Expr::String(StringExpr::Literal("".to_string().into())),
        Expr::Integer(IntegerExpr::Literal(0)),
        Expr::Float(FloatExpr::Literal(20.)),
        Expr::Float(FloatExpr::Literal(0.5)),
        Expr::Float(FloatExpr::Literal(0.5)),
        Expr::Float(FloatExpr::Literal(0.5)),
    ];

    ClassDeclaration {
        properties,
        methods,
        property_default_expressions,
    }
}
