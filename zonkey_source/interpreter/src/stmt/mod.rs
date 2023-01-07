use crate::{expr::Expr, token::Token, tree_walker::value::ValueType};

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Exit,
    VariableDeclaration(ValueType, String, Expr),
    VariableAssignment(String, Expr, Token),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Loop(Box<Stmt>),
    Break,
    Continue,
}
