use crate::{expr::Expr, token::Token, tree_walker::value::ValueType};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Exit,
    VariableDeclaration(ValueType, String, Expr),
    VariableAssignment(String, Expr, Token),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Loop(Box<Stmt>),
    FunctionDeclaration(String, Vec<(ValueType, String)>, Box<Stmt>),
    Start(Box<Stmt>),
    Break,
    Continue,
}
