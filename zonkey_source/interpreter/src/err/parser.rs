use crate::token::Token;

pub struct ParserErr {
    pub errors: Vec<ParserErrType>,
}

impl ParserErr {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn add(&mut self, error_type: ParserErrType) {
        self.errors.push(error_type);
    }

    pub fn had_error(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn get_length(&self) -> usize {
        self.errors.len()
    }
}

pub enum ParserErrType {
    // Miscellaneous/Global errors
    UnterminatedStatement(Token, Option<Token>),
    UnexpectedTokenInGlobal(Token),

    // Start declaration errors
    NoStartBlock,
    RedefinedStart(Token, Token),

    // If statement errors
    IfExpectedLeftParen(Token, Option<Token>),
    IfExpectedRightParen(Token, Option<Token>),
    IfConditionNotBool(Token),

    // While statement errors
    WhileExpectedLeftParen(Token, Option<Token>),
    WhileExpectedRightParen(Token, Option<Token>),
    WhileConditionNotBool(Token),

    // For statement errors
    ForExpectedLeftParen(Token, Option<Token>),
    ForExpectedRightParen(Token, Option<Token>),
    ForExpectedComma1(Token, Option<Token>),
    ForExpectedComma2(Token, Option<Token>),
    ForConditionNotBool(Token),

    // Function declaration errors
    FunctionDeclarationExpectedName(Token, Option<Token>),
    FunctionDeclarationExpectedLeftParen(Token, Option<Token>),
    FunctionDeclarationExpectedParameterName(Token, Option<Token>),
    FunctionDeclarationExpectedParameterType(Token, Option<Token>),
    FunctionDeclarationExpectedCommaOrRightParen(Token, Option<Token>),
    FunctionDeclarationExpectedReturnType(Token, Option<Token>),
}
