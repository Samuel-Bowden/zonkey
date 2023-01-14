use crate::{literal::Literal, token::Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Literal),
    Variable(String),
    Call(String, Vec<Expr>),
}
