use std::fmt::Display;

#[derive(Debug)]
pub enum LexerErr {
    UnexpectedGrapheme(u64, String),
    UnterminatedString(u64),
    FloatMoreThanOneDecimalPoint,
}

impl Display for LexerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedGrapheme(line, grapheme) => write!(f, "Unexpected grapheme ({grapheme}) found on line {line}"),
            Self::UnterminatedString(line) => write!(f, "Unterminated string on line {line}"),
            Self::FloatMoreThanOneDecimalPoint => write!(f, "Float has more than one decimal point"),
        }
    }
}
