use crate::token::Token;

#[derive(Debug)]
pub enum TreeWalkerErr {
    PropertyNotInitialised(Token),
    IndexOutOfRange,
    DivisionByZero,
    FailedStringToIntegerCast,
    FailedStringToFloatCast,
    FailedStringToBooleanCast,
    Exit,
    ReadAddressFailed(String),
    WriteAddressFailed(String),
    InvalidHexColour(String),
}
