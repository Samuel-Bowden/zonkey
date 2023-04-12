use crate::{parser::value::ValueType, token::Token};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ParserErrType {
    // Miscellaneous/Global errors
    UnterminatedStatement(Token, Option<Token>),
    UnexpectedTokenInGlobal(Token),
    VariableNotFound(Token, String),
    ExpectedValue(Token, Option<Token>),
    DeclarationInvalidReturnExpressionType(Token, Option<ValueType>, Option<ValueType>),
    BreakOutsideLoop(Token),
    ContinueOutsideLoop(Token),

    // Block errors
    BlockExpectedLeftBrace(Token, Option<Token>),
    BlockExpectedRightBrace(Token),

    // Start errors
    NoStartBlock,
    RedefinedStart(Token, Token),

    // Call errors
    CallExpectedCommaOrRightParen(Token, Option<Token>),
    CallIncorrectArgumentsNum(Token, usize, usize, String),
    CallArgumentIncorrectType(Token, usize, Option<ValueType>, String),
    CallNotFound(Token, String),

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
    FunctionRedeclared(Token),
    FunctionDeclarationExpectedName(Token, Option<Token>),
    FunctionDeclarationExpectedLeftParen(Token, Option<Token>),
    FunctionDeclarationExpectedParameterName(Token, Option<Token>),
    FunctionDeclarationExpectedParameterType(Token, Option<Token>),
    FunctionDeclarationExpectedCommaOrRightParen(Token, Option<Token>),
    FunctionDeclarationExpectedReturnType(Token, Option<Token>),
    FunctionDeclarationDidNotReturnValueInAllCases(Token, ValueType),

    // Operator errors
    InvalidAssignmentOperator(Token, ValueType),
    UnmatchingTypesAssignmentOperatator(Token, Option<ValueType>, Option<ValueType>),

    // Variable declaration errors
    VariableDeclarationExpectedName(Token, Option<Token>),
    VariableDeclarationAlreadyDeclared(Token, String),
    VariableDeclarationExpectedEqual(Token, Option<Token>),
    VariableDeclarationExprEvalNone(usize, usize),

    // Comparision errors
    ComparisionUnmatchingTypes(Token, Option<ValueType>, Option<ValueType>),
    ComparisionInvalidForType(Token, Option<ValueType>),

    // Operator errors
    OperatorUnmatchingTypes(Token, Option<ValueType>, Option<ValueType>),
    OperatorInvalidForType(Token, Option<ValueType>),

    // Grouping errors
    GroupingExpectedRightParen(Token, Option<Token>),

    // Unary operator errors
    UnaryOperatorInvalidForType(Token, Option<ValueType>),

    // Casting errors
    CastNotPossible(Token, Option<ValueType>, Option<ValueType>),
    CastPointless(Token, Option<ValueType>),

    // Class declaration errors
    ClassRedeclared(Token),
    ClassNotFound(Token),
    ClassDeclarationExpectedName(Token, Option<Token>),
    ClassDeclarationExpectedLeftBrace(Token, Option<Token>),
    ClassDeclarationExpectedRightBrace(Token, Token),
    ClassDeclarationExpectedPropertyName(Token, Option<Token>),
    ClassDeclarationExpectedMethodName(Token, Option<Token>),
    ClassDeclarationUnterminatedProperty(Token, Option<Token>),
    ClassDeclarationRedeclaredProperty(Token, String),
    ClassDeclarationRedeclaredConstructor(Token),
    ClassDeclarationRedeclaredMethod(Token, String),
    ClassDeclarationNoConstructor(Token),
    ClassDeclarationExpectPropertyTop(Token),

    // Method call errors
    MethodCallExpectedName(Token, Option<Token>),
    MethodCallExpectedLeftParen(Token, Option<Token>),
    MethodCallNotObject(Token, Option<ValueType>),

    // Property accessor errors
    PropertyAccessorExpectedName(Token, Option<Token>),
    PropertyNotFound(Token, String),
    PropertyAccessorOutsideClass(Token, String),
}
