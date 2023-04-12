use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum LexerErr {
    UnexpectedGrapheme(usize),
    UnterminatedString(usize),
    FloatMoreThanOneDecimalPoint(usize),
    FailedToParseFloat(usize, usize, ParseFloatError),
    FailedToParseInteger(usize, usize, ParseIntError),
}
