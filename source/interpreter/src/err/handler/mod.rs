use self::err_reporter::ErrReporter;
use super::InterpreterErr;

mod err_reporter;
mod lexer;
mod parser;
mod tree_walker;

pub fn run(error: InterpreterErr, graphemes: &Vec<&str>) -> String {
    let mut err_reporter = ErrReporter::new(graphemes);

    match error {
        InterpreterErr::LexerFailed(err) => lexer::err_handler(&mut err_reporter, err),
        InterpreterErr::ParserFailed(err) => parser::err_handler(&mut err_reporter, err),
        InterpreterErr::TreeWalkerFailed(err) => tree_walker::err_handler(&mut err_reporter, err),
    }

    err_reporter.stderr
}
