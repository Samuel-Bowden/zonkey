use crate::{
    expr::{Expr, IntegerExpr, StringExpr},
    parser::{declaration::ClassDeclaration, value::Value},
};
use rustc_hash::FxHashMap;
use std::rc::Rc;

pub fn new(_: Rc<String>) -> ClassDeclaration {
    let mut properties = FxHashMap::default();

    properties.insert("text".to_string().into(), Value::String(0));
    properties.insert("link".to_string().into(), Value::String(1));
    properties.insert("id".to_string().into(), Value::Integer(0));

    let methods = FxHashMap::default();

    let property_default_expressions = vec![
        Expr::String(StringExpr::Literal("".to_string().into())),
        Expr::String(StringExpr::Literal("".to_string().into())),
        Expr::Integer(IntegerExpr::Literal(0)),
    ];

    ClassDeclaration {
        properties,
        methods,
        property_default_expressions,
    }
}
