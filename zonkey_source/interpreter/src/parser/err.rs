use std::fmt::Display;

#[derive(Debug)]
pub enum ParserErr {
    ExpectedLiteral,
    UnterminatedStatement,
    ParserNotReachedEOF,
    PrintMissingLeftParen,
    PrintMissingRightParen,
    IfMissingLeftParen,
    IfMissingRightParen,
    WhileMissingLeftParen,
    WhileMissingRightParen,
    ExitMissingLeftParen,
    ExitMissingRightParen,
    ExpectedVariableName,
    ExpectedVariableEqual,
    LeftValueNotVariable,
    ExpectedLeftBraceBeforeBlock,
    ExpectedRightBraceAfterBlock,
    NoDataTypeForVariableDeclaration,
    ForMissingLeftParen,
    ForMissingRightParen,
    ForMissingCommaAfterInitialiserStatement,
    ForMissingCommaAfterTestStatement,
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
            Self::IfMissingLeftParen => {
                write!(f, "If statement is missing '(` to wrap condition")
            }
            Self::IfMissingRightParen => {
                write!(f, "If statement is missing ')` to wrap condition")
            }
            Self::WhileMissingLeftParen => {
                write!(f, "While statement is missing '(` to wrap condition")
            }
            Self::WhileMissingRightParen => {
                write!(f, "While statement is missing ')` to wrap condition")
            }
            Self::ExitMissingLeftParen => {
                write!(f, "Exit statement is missing '('")
            }
            Self::ExitMissingRightParen => {
                write!(f, "Exit statement is missing ')'")
            }
            Self::ExpectedVariableName => {
                write!(f, "Expected variable name")
            }
            Self::ExpectedVariableEqual => {
                write!(f, "Expected '=' after variable declaration - variables must be initialised with an expression")
            }
            Self::LeftValueNotVariable => {
                write!(f, "Left hand side of assignment must be a variable name")
            }
            Self::ExpectedLeftBraceBeforeBlock => {
                write!(f, "Expected '{{' before block")
            }
            Self::ExpectedRightBraceAfterBlock => {
                write!(f, "Expected '}}' after block")
            }
            Self::NoDataTypeForVariableDeclaration => {
                write!(f, "No data type given for variable declaration")
            }
            Self::ForMissingLeftParen => {
                write!(f, "For statement is missing '(` to wrap clauses")
            }
            Self::ForMissingRightParen => {
                write!(f, "For statement is missing ')` to wrap clauses")
            }
            Self::ForMissingCommaAfterInitialiserStatement => {
                write!(f, "For statement is missing ',' to separate initialiser statement from test statement")
            }
            Self::ForMissingCommaAfterTestStatement => {
                write!(
                    f,
                    "For statement is missing ',' to separate test statement from update statement"
                )
            }
        }
    }
}
