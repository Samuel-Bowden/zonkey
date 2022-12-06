use std::fmt::Display;

#[derive(Debug)]
pub enum ParserErr {
    ExpectedLiteral,
}

impl Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedLiteral => write!(f, "Expected literal"),
        }
    }
}
