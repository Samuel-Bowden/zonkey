use crate::expr::{BooleanExpr, FloatExpr, IntegerExpr, StringExpr};

pub mod prompt;

#[derive(Debug)]
pub enum CliFunctionNone {
    PrintLineInteger(Box<IntegerExpr>),
    PrintLineFloat(Box<FloatExpr>),
    PrintLineString(Box<StringExpr>),
    PrintLineBoolean(Box<BooleanExpr>),
    PrintLine,
    PrintInteger(Box<IntegerExpr>),
    PrintFloat(Box<FloatExpr>),
    PrintString(Box<StringExpr>),
    PrintBoolean(Box<BooleanExpr>),
}

#[derive(Debug)]
pub enum CliFunctionString {
    Prompt(Box<StringExpr>),
}

#[derive(Debug)]
pub enum CliFunctionInteger {
    Prompt(Box<StringExpr>),
}
