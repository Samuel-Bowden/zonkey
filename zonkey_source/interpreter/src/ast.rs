use crate::stmt::Stmt;

#[derive(Debug)]
pub struct AST {
    pub start: Stmt,
    pub callable: Vec<Stmt>,
}
