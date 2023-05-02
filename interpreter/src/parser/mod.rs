pub mod declaration;
pub mod err;
mod location;
mod production;
mod status;
pub mod value;

use crate::{
    ast::AST,
    parser::declaration::{CallableDeclaration, ClassDeclaration},
    parser::location::Location,
    parser_debug, standard_prelude,
    stmt::Stmt,
    token::Token,
};
use err::{ParserErr, ParserErrType};
use rustc_hash::FxHashMap;
use std::rc::Rc;
use value::ValueType;

pub struct Parser {
    tokens: Vec<Token>,
    environments: Vec<FxHashMap<Rc<String>, Location>>,
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
    current_properties: Option<FxHashMap<Rc<String>, Location>>,
    callables: Vec<Rc<Stmt>>,
    error: ParserErr,
    start_definition: Option<(Token, Option<Stmt>)>,
    current: usize,
    sub_expression_limit: usize,
    nested_scope_limit: usize,
}

impl Parser {
    pub fn run(tokens: Vec<Token>) -> Result<AST, ParserErr> {
        let mut parser = Self {
            tokens,
            environments: vec![],
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
            sub_expression_limit: 0,
            nested_scope_limit: 0,
        };

        parser_debug!("Production rule path:");

        parser.program();

        match (parser.start_definition, parser.error.had_error()) {
            (Some((_, Some(stmt))), false) => {
                let ast = AST {
                    start: stmt,
                    callable: parser.callables,
                };

                parser_debug!("AST");
                #[cfg(debug_assertions)]
                println!("{:#?}", ast);

                Ok(ast)
            }
            (t, _) => {
                if let None = t {
                    parser.error.add(ParserErrType::NoStartBlock)
                }
                Err(parser.error)
            }
        }
    }
}
