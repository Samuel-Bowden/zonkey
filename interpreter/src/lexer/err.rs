use std::num::ParseIntError;

#[derive(Debug)]
pub enum LexerErr {
    UnexpectedGrapheme(usize),
    UnterminatedString(usize),
    FloatMoreThanOneDecimalPoint(usize),
    FailedToParseInteger(usize, usize, ParseIntError),
}
