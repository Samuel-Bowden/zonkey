use crate::token::Token;

#[derive(Debug)]
pub enum TreeWalkerErr {
    PropertyNotInitialised(Token),
    IndexOutOfRange(usize, usize, Token),
    DivisionByZero(Token),
    FailedStringToIntegerCast(Token),
    FailedStringToFloatCast(Token),
    Exit,
    InsufficientPermissionLevel,
    InstallFailed(String),
    SettingsFailed(String),
    ReadAddressFailed(String),
    WriteAddressFailed(String),
    InvalidHexColour(String),
}
