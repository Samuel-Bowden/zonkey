use self::err_reporter::ErrReporter;
use interpreter::err::InterpreterErr;

mod err_reporter;
mod lexer;
mod parser;

pub fn run(error: InterpreterErr, graphemes: &Vec<&str>) {
    let err_reporter = ErrReporter::new(graphemes);

    match error {
        InterpreterErr::LexerFailed(err) => lexer::err_handler(err_reporter, err),
        InterpreterErr::ParserFailed(err) => parser::err_handler(err_reporter, err),
    }
}
