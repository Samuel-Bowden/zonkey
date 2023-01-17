use crate::token::Token;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParserErr {
    ExpectedLiteral(Option<Token>),
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
    ExpectedVariableName(Option<Token>),
    ExpectedVariableEqual,
    LeftValueNotVariable,
    ExpectedLeftBraceBeforeBlock,
    ExpectedRightBraceAfterBlock,
    VariableDeclarationBadValueType,
    ForMissingLeftParen,
    ForMissingRightParen,
    ForMissingCommaAfterInitialiserStatement,
    ForMissingCommaAfterTestStatement,
    FunctionDeclarationMissingName,
    FunctionDeclarationMissingLeftParen,
    FunctionDeclarationMissingRightParen,
    FunctionDeclarationParameterBadDataType,
    FunctionDeclarationParameterMissingName,
    FunctionDeclarationExpectedCommaOrRightParen,
    CallMissingArgument,
    CallExpectedCommaOrRightParen,
    CallToUndeclaredFunction(String),
}

impl Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedLiteral(found) => write!(f, "Expected literal, found {:?}", found),
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
            Self::ExpectedVariableName(other) => {
                write!(f, "Expected variable name, found {:?}", other)
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
            Self::VariableDeclarationBadValueType => {
                write!(
                    f,
                    "Variable declaration has a missing or invalid value type"
                )
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
            Self::FunctionDeclarationMissingName => {
                write!(f, "Function declaration is missing a name")
            }
            Self::FunctionDeclarationMissingLeftParen => {
                write!(f, "Function declaration is missing '(' to wrap parameters")
            }
            Self::FunctionDeclarationMissingRightParen => {
                write!(f, "Function declaration is missing ')' to wrap parameters")
            }
            Self::FunctionDeclarationParameterBadDataType => {
                write!(
                    f,
                    "Parameter has a missing or invalid data type in function declaration"
                )
            }
            Self::FunctionDeclarationParameterMissingName => {
                write!(f, "Parameter is missing a name in function declaration")
            }
            Self::FunctionDeclarationExpectedCommaOrRightParen => {
                write!(f, "Expected ')' to finish list of parameters or ',' to add another parameter for the function declaration")
            }
            Self::CallMissingArgument => {
                write!(f, "Expected an argument in function call")
            }
            Self::CallExpectedCommaOrRightParen => {
                write!(f, "Expected ')' to finish list of arguments or ',' to add another argument for the call")
            }
            Self::CallToUndeclaredFunction(name) => {
                write!(f, "Call to undeclared function {name}")
            }
        }
    }
}
