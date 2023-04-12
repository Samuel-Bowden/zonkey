pub mod declaration;
mod production;
mod status;
pub mod value;

use self::value::ValueType;
use crate::{
    ast::AST,
    err::parser::{ParserErr, ParserErrType},
    parser::declaration::{CallableDeclaration, ClassDeclaration},
    parser::value::Value,
    parser_debug, standard_prelude,
    stmt::Stmt,
    token::Token,
};
use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    value_stack: Vec<IndexMap<Rc<String>, Value>>,
    integer_next_id: usize,
    float_next_id: usize,
    string_next_id: usize,
    boolean_next_id: usize,
    object_next_id: usize,
    function_declarations: FxHashMap<Rc<String>, CallableDeclaration>,
    class_declarations: FxHashMap<Rc<String>, ClassDeclaration>,
    current_return_type: Option<ValueType>,
    returned_value: bool,
    loop_count: usize,
    current_properties: Option<FxHashMap<Rc<String>, Value>>,
    callables: Vec<Rc<Stmt>>,
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
            object_next_id: 0,
            function_declarations: standard_prelude::functions::new(),
            class_declarations: standard_prelude::classes::new(),
            current_return_type: None,
            returned_value: false,
            loop_count: 0,
            current_properties: None,
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
            (Some((_, Some(stmt))), false) => {
                let ast = AST {
                    start: stmt,
                    callable: self.callables,
                };

                parser_debug!("AST");
                #[cfg(debug_assertions)]
                println!("{:#?}", ast);

                Ok(ast)
            }
            (t, _) => {
                if let None = t {
                    self.error.add(ParserErrType::NoStartBlock)
                }
                Err(self.error)
            }
        }
    }
}
