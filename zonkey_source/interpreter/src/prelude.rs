use crate::expr::{Expr, StringExpr};

#[derive(Debug)]
pub enum NativeFunctionNone {
    Print(Box<Expr>, bool),
}

#[derive(Debug)]
pub enum NativeFunctionString {
    Prompt(Box<StringExpr>),
}
