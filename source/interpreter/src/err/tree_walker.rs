use crate::token::Token;

#[derive(Debug)]
pub enum TreeWalkerErr {
    PropertyNotInitialised(Token),
    DivisionByZero,
    FailedStringToIntegerCast,
    FailedStringToFloatCast,
    FailedStringToBooleanCast,
    Exit,
    ReadAddressFailed(String),
    WriteAddressFailed(String),
}
