use std::fmt::Display;

#[derive(Debug)]
pub enum ParserErr {
    ExpectedLiteral,
    UnterminatedStatement,
    ParserNotReachedEOF,
    PrintMissingLeftParen,
    PrintMissingRightParen,
    ExitMissingLeftParen,
    ExitMissingRightParen,
}

impl Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedLiteral => write!(f, "Expected literal"),
            Self::UnterminatedStatement => write!(f, "Unterminated statement"),
            Self::ParserNotReachedEOF => write!(f, "Parser failed to process all source code"),
            Self::PrintMissingLeftParen => {
                write!(f, "Print statement is missing '(' to wrap expression")
            }
            Self::PrintMissingRightParen => {
                write!(f, "Print statement is missing ')' to wrap expression")
            }
            Self::ExitMissingLeftParen => {
                write!(f, "Exit statement is missing '('")
            }
            Self::ExitMissingRightParen => {
                write!(f, "Exit statement is missing ')'")
            }
        }
    }
}
