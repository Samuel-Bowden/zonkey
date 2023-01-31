use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use crate::{environment::Environment, parser::Parser, tree_walker::TreeWalker};
use function::Function;
use stmt::Stmt;

mod assignment_operator;
mod comparison;
mod debugger;
mod environment;
pub mod err;
mod expr;
mod function;
mod function_declaration;
mod lexer;
mod native_function;
mod operator;
mod parser;
pub mod return_type;
mod start;
mod stmt;
pub mod token;
mod tree_walker;
pub mod value_type;

pub fn run(source: &Vec<&str>) -> Result<(), InterpreterErr> {
    interpreter_debug!("Debug build");

    let tokens = run_lexer(source)?;

    let (start, functions) = run_parser(tokens)?;

    run_tree_walker(start, functions);

    Ok(())
}

fn run_lexer(source: &Vec<&str>) -> Result<Vec<Token>, InterpreterErr> {
    interpreter_debug!("Starting lexer");

    let lexer = Lexer::new(source).run();

    match lexer {
        Ok(lexer) => {
            interpreter_debug!("Lexer finished successfully");
            Ok(lexer.tokens)
        }
        Err(e) => Err(InterpreterErr::LexerFailed(e)),
    }
}

fn run_parser(tokens: Vec<Token>) -> Result<(Stmt, Vec<Function>), InterpreterErr> {
    interpreter_debug!("Starting parser");

    match Parser::new(tokens).run() {
        Ok((start, functions)) => {
            interpreter_debug!("Parser completed successfully");
            Ok((start, functions))
        }
        Err(e) => Err(InterpreterErr::ParserFailed(e)),
    }
}

fn run_tree_walker(start: Stmt, functions: Vec<Function>) {
    interpreter_debug!("Starting tree walker");

    let environment = Environment::new();

    TreeWalker::new(&functions, environment).interpret(&start);
}
