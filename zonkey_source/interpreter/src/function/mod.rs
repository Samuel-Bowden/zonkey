use crate::{expr::Expr, stmt::Stmt};

#[derive(Debug)]
pub struct Function {
    pub start: Stmt,
    pub return_expr: Option<Expr>,
}
