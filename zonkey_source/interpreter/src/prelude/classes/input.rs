use crate::{
    expr::{BooleanExpr, Expr, IntegerExpr, StringExpr},
    parser::{
        declaration::{CallableDeclaration, CallableType, ClassDeclaration},
        value::{Value, ValueType},
    },
};
use rustc_hash::FxHashMap;
use std::rc::Rc;

pub fn new(_: Rc<String>) -> ClassDeclaration {
    let mut properties = FxHashMap::default();

    properties.insert("placeholder".to_string().into(), Value::String(0));
    properties.insert("text".to_string().into(), Value::String(1));
    properties.insert("id".to_string().into(), Value::Integer(0));
    properties.insert("confirmed".to_string().into(), Value::Boolean(0));

    let mut methods = FxHashMap::default();

    methods.insert(
        "confirmed".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Boolean),
        }),
    );

    methods.insert(
        "get_text".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::String),
        }),
    );

    let property_default_expressions = vec![
        Expr::String(StringExpr::Literal("".to_string().into())),
        Expr::String(StringExpr::Literal("".to_string().into())),
        Expr::Integer(IntegerExpr::Literal(0)),
        Expr::Boolean(BooleanExpr::Literal(false)),
    ];

    ClassDeclaration {
        properties,
        methods,
        property_default_expressions,
    }
}
