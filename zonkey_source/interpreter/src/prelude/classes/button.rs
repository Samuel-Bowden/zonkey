use crate::{
    expr::{BooleanExpr, Expr, FloatExpr, IntegerExpr, StringExpr},
    parser::{
        declaration::{CallableDeclaration, CallableType, ClassDeclaration},
        value::{Value, ValueType},
    },
};
use rustc_hash::FxHashMap;
use std::rc::Rc;

pub fn new(button: Rc<String>) -> ClassDeclaration {
    let mut properties = FxHashMap::default();

    properties.insert("text".to_string().into(), Value::String(0));
    properties.insert("id".to_string().into(), Value::Integer(0));
    properties.insert("clicked".to_string().into(), Value::Boolean(0));
    properties.insert("bg_red".to_string().into(), Value::Float(0));
    properties.insert("bg_green".to_string().into(), Value::Float(1));
    properties.insert("bg_blue".to_string().into(), Value::Float(2));

    let mut methods = FxHashMap::default();

    methods.insert(
        "set_text".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::clone(&button))),
        }),
    );

    methods.insert(
        "set_background_colour".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float, ValueType::Float, ValueType::Float],
            return_type: Some(ValueType::Class(Rc::clone(&button))),
        }),
    );

    methods.insert(
        "clicked".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Boolean),
        }),
    );

    let property_default_expressions = vec![
        Expr::String(StringExpr::Literal("".to_string().into())),
        Expr::Integer(IntegerExpr::Literal(0)),
        Expr::Boolean(BooleanExpr::Literal(false)),
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
