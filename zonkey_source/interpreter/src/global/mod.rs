use crate::{
    function::{Function, ZonkeyFunction},
    stmt::Stmt,
    tree_walker::err::TreeWalkerErr,
};
use std::collections::{HashMap, VecDeque};

mod native_functions;

pub struct Global {
    pub functions: HashMap<String, Box<dyn Function>>,
}

impl Global {
    pub fn new() -> Self {
        let mut functions = HashMap::new();

        native_functions::insert(&mut functions);

        Self { functions }
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
                Stmt::FunctionDeclaration(name, parameters, block) => {
                    self.insert_function(
                        name.clone(),
                        Box::new(ZonkeyFunction {
                            block,
                            parameters,
                            name,
                        }),
                    );
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

    pub fn get_function(&self, name: &String) -> Option<&Box<dyn Function>> {
        if let Some(function) = self.functions.get(name) {
            return Some(function);
        }

        None
    }

    pub fn insert_function(&mut self, name: String, function: Box<dyn Function>) {
        self.functions.insert(name, function);
    }
}
