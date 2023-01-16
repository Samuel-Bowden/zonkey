use crate::{stmt::Stmt, tree_walker::err::TreeWalkerErr};
use std::collections::VecDeque;

mod native_functions;

pub struct Global {}

impl Global {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan_global(
        &mut self,
        mut statements: VecDeque<Stmt>,
    ) -> Result<Box<Stmt>, TreeWalkerErr> {
        let mut start_block = None;

        // Scan global scope for start blocks and functions
        while let Some(statement) = statements.pop_front() {
            match statement {
                Stmt::Start(block) => {
                    if let Some(_) = start_block {
                        return Err(TreeWalkerErr::MultipleStartDeclarations);
                    } else {
                        start_block = Some(block);
                    }
                }
                _ => return Err(TreeWalkerErr::InvalidCodeInGlobalScope),
            }
        }

        if let Some(start_block) = start_block {
            Ok(start_block)
        } else {
            Err(TreeWalkerErr::NoStartDeclaration)
        }
    }
}
