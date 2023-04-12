#[derive(Debug)]
pub enum TreeWalkerErr {
    DivisionByZero,
    FailedStringToIntegerCast,
    FailedStringToFloatCast,
    FailedStringToBooleanCast,
    Exit,
    ReadAddressFailed(String),
    WriteAddressFailed(String),
}
