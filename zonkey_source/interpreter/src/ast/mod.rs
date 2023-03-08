use crate::stmt::Stmt;

pub struct AST {
    pub start: Stmt,
    pub callable: Vec<Stmt>,
}
