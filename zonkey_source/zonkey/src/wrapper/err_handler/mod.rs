use self::err_reporter::ErrReporter;
use interpreter::err::InterpreterErr;

mod err_reporter;
mod lexer;
mod parser;
mod tree_walker;

pub fn run(error: InterpreterErr, graphemes: &Vec<&str>) {
    let err_reporter = ErrReporter::new(graphemes);

    match error {
        InterpreterErr::LexerFailed(err) => lexer::err_handler(err_reporter, err),
        InterpreterErr::ParserFailed(err) => parser::err_handler(err_reporter, err),
        InterpreterErr::TreeWalkerFailed(err) => tree_walker::err_handler(err_reporter, err),
    }
}
