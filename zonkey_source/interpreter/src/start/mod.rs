use crate::{stmt::Stmt, token::Token};

#[derive(Debug)]
pub struct Start {
    pub stmt: Option<Stmt>,
    pub token: Token,
}
