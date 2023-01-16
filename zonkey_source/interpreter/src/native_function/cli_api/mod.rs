use crate::expr::{StringExpr, IntegerExpr, FloatExpr, BooleanExpr};

pub mod prompt;

#[derive(Debug)]
pub enum CliFunctionNone {
    PrintLineInteger(Box<IntegerExpr>),
    PrintLineFloat(Box<FloatExpr>),
    PrintLineString(Box<StringExpr>),
    PrintLineBoolean(Box<BooleanExpr>),
    PrintInteger(Box<IntegerExpr>),
    PrintFloat(Box<FloatExpr>),
    PrintString(Box<StringExpr>),
    PrintBoolean(Box<BooleanExpr>),
}

#[derive(Debug)]
pub enum CliFunctionString {
    Prompt(Box<StringExpr>),
}
