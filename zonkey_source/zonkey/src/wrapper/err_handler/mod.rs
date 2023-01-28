use interpreter::err::InterpreterErr;
use self::err_reporter::ErrReporter;

mod lexer;
mod parser;
mod err_reporter;

pub fn run(error: InterpreterErr, graphemes: &Vec<&str>) {
    let err_reporter = ErrReporter::new(graphemes);

    match error {
        InterpreterErr::LexerFailed(err) => lexer::err_handler(err_reporter, err),
        InterpreterErr::ParserFailed(err) => parser::err_handler(err_reporter, err),
    }
}
