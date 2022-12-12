use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Exit,
}
