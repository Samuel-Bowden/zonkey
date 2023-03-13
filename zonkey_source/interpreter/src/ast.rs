use crate::stmt::Stmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct AST {
    pub start: Stmt,
    pub callable: Vec<Rc<Stmt>>,
}
