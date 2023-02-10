use crate::{return_type::ReturnType, token::Token, value_type::ValueType};

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
    VariableNotFound(Token, String),
    ExpectedLiteralVariableCall(Token, Option<Token>),

    // Block errors
    BlockExpectedLeftBrace(Token, Option<Token>),
    BlockExpectedRightBrace(Token, Token),

    // Start errors
    NoStartBlock,
    RedefinedStart(Token, Token),
    StartCannotReturn(Token),

    // Call errors
    CallExpectedCommaOrRightParen(Token, Option<Token>),
    CallIncorrectArgumentsNum(Token, usize, usize, String),
    CallArgumentIncorrectType(Token, usize, ReturnType, String),
    CallModuleFunctionNotFound(Token, String, String),
    CallModuleNotFound(Token, String),
    CallFunctionNotFound(Token, String),

    // If statement errors
    IfExpectedLeftParen(Token, Option<Token>),
    IfExpectedRightParen(Token, Option<Token>),
    IfConditionNotBool(usize, usize),

    // While statement errors
    WhileExpectedLeftParen(Token, Option<Token>),
    WhileExpectedRightParen(Token, Option<Token>),
    WhileConditionNotBool(usize, usize),

    // For statement errors
    ForExpectedLeftParen(Token, Option<Token>),
    ForExpectedRightParen(Token, Option<Token>),
    ForExpectedComma1(Token, Option<Token>),
    ForExpectedComma2(Token, Option<Token>),
    ForConditionNotBool(usize, usize),

    // Function declaration errors
    FunctionDeclarationExpectedName(Token, Option<Token>),
    FunctionDeclarationExpectedLeftParen(Token, Option<Token>),
    FunctionDeclarationExpectedParameterName(Token, Option<Token>),
    FunctionDeclarationExpectedParameterType(Token, Option<Token>),
    FunctionDeclarationExpectedCommaOrRightParen(Token, Option<Token>),
    FunctionDeclarationExpectedReturnType(Token, Option<Token>),
    FunctionDeclarationInvalidReturnExpressionType(Token, ReturnType, ReturnType),

    // Operator errors
    InvalidAssignmentOperator(Token, ValueType),
    UnmatchingTypesAssignmentOperatator(Token, ReturnType, ReturnType),

    // Variable declaration errors
    VariableDeclarationExpectedName(Token, Option<Token>),
    VariableDeclarationAlreadyDeclared(Token, String),
    VariableDeclarationExpectedEqual(Token, Option<Token>),
    VariableDeclarationExprEvalNone(usize, usize),

    // Comparision errors
    ComparisionUnmatchingTypes(Token, ReturnType, ReturnType),
    ComparisionInvalidForType(Token, ReturnType),

    // Operator errors
    OperatorUnmatchingTypes(Token, ReturnType, ReturnType),
    OperatorInvalidForType(Token, ReturnType),

    // Module errors
    ModuleExpectedIdentifier(Token, Option<Token>),
    ModuleExpectedLeftParen(Token, Option<Token>),

    // Grouping errors
    GroupingExpectedRightParen(Token, Option<Token>),

    // Unary operator errors
    UnaryOperatorInvalidForType(Token, ReturnType),

    // Casting errors
    CastNotPossible(Token, ReturnType, ReturnType),
    CastPointless(Token, ReturnType),
}
