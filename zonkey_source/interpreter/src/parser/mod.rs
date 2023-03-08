mod declaration;
mod production;
mod status;
pub mod value;

use crate::{
    ast::AST,
    err::parser::{ParserErr, ParserErrType},
    parser::declaration::{ClassDeclaration, FunctionDeclaration},
    parser::value::Value,
    parser_debug,
    stmt::Stmt,
    token::Token,
};
use indexmap::IndexMap;
use rustc_hash::FxHashMap;

pub struct Parser {
    tokens: Vec<Token>,
    value_stack: Vec<IndexMap<String, Value>>,
    integer_next_id: usize,
    float_next_id: usize,
    string_next_id: usize,
    boolean_next_id: usize,
    function_declarations: FxHashMap<String, FunctionDeclaration>,
    class_declarations: FxHashMap<String, ClassDeclaration>,
    current_function_declaration: Option<FunctionDeclaration>,
    callables: Vec<Stmt>,
    error: ParserErr,
    start_definition: Option<(Token, Option<Stmt>)>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            value_stack: vec![],
            integer_next_id: 0,
            float_next_id: 0,
            string_next_id: 0,
            boolean_next_id: 0,
            function_declarations: FxHashMap::default(),
            class_declarations: FxHashMap::default(),
            current_function_declaration: None,
            callables: vec![],
            error: ParserErr::new(),
            start_definition: None,
            current: 0,
        }
    }

    pub fn run(mut self) -> Result<AST, ParserErr> {
        parser_debug!("Production rule path:");

        self.program();

        match (self.start_definition, self.error.had_error()) {
            (Some((_, Some(stmt))), false) => Ok(AST {
                start: stmt,
                callable: self.callables,
            }),
            (t, _) => {
                if let None = t {
                    self.error.add(ParserErrType::NoStartBlock)
                }
                Err(self.error)
            }
        }
    }
}
