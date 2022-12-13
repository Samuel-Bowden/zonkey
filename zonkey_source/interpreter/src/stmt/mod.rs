use crate::{expr::Expr, tree_walker::value::ValueType};

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Exit,
    VariableDeclaration(ValueType, String, Expr),
    VariableAssignment(String, Expr),
}
