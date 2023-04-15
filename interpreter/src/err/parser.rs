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
    BreakOutsideLoop(Token),
    ContinueOutsideLoop(Token),
    CannotCreateVariableCalledSelf(Token),

    // Callable Declaration errors
    DeclarationInvalidReturnExpressionType(Token, Option<ValueType>, Option<ValueType>),
    DeclarationDidNotReturnValueInAllCases(Token, ValueType),
    DeclarationExpectedParameterName(Token, Option<Token>),
    DeclarationExpectedParameterType(Token, Option<Token>),
    DeclarationExpectedCommaOrRightParen(Token, Option<Token>),
    DeclarationExpectedReturnType(Token, Option<Token>),
    DeclarationExpectedLeftParen(Token, Option<Token>),

    // Array errors
    ArrayNonMatchingValue(Token, usize, ValueType, Option<ValueType>),
    ArrayExpectedCommaOrRightBracket(Token, Option<Token>),
    ArrayEmptyType(Token, Option<Token>),
    ArrayTypeNotClosed(Token, Option<Token>),

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
    ForExpectedLet(Token, Option<Token>),
    ForExpectedLeftParen(Token, Option<Token>),
    ForExpectedRightParen(Token, Option<Token>),
    ForExpectedComma1(Token, Option<Token>),
    ForExpectedComma2(Token, Option<Token>),
    ForConditionNotBool(usize, usize),

    // Function declaration errors
    FunctionRedeclared(Token),
    FunctionDeclarationExpectedName(Token, Option<Token>),

    // Variable declaration errors
    VariableDeclarationExpectedName(Token, Option<Token>),
    VariableDeclarationAlreadyDeclared(Token, String),
    VariableDeclarationExpectedEqual(Token, Option<Token>),
    VariableDeclarationExprEvalNone(usize, usize),

    // Comparision errors
    ComparisionUnmatchingTypes(Token, Option<ValueType>, Option<ValueType>),
    ComparisionInvalidForType(Token, Option<ValueType>),

    // Binary Operator errors
    OperatorUnmatchingTypes(Token, Option<ValueType>, Option<ValueType>),
    OperatorInvalidForType(Token, Option<ValueType>),

    // Assignment Operator errors
    InvalidAssignmentOperator(Token, ValueType),
    UnmatchingTypesAssignmentOperatator(Token, Option<ValueType>, Option<ValueType>),

    // Grouping errors
    GroupingExpectedRightParen(Token, Option<Token>),

    // Unary operator errors
    UnaryOperatorInvalidForType(Token, Option<ValueType>),

    // Class declaration errors
    ClassRedeclared(Token),
    ClassNotFound(Token),
    ClassDeclarationExpectedName(Token, Option<Token>),
    ClassDeclarationExpectedLeftBrace(Token, Option<Token>),
    ClassDeclarationExpectedRightBrace(Token, Option<Token>),
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
    MethodCallNotFound(Token, String, String),

    // Property accessor errors
    PropertyAccessorExpectedName(Token, Option<Token>),
    PropertyNotFound(Token, String),
    PropertyAccessorOutsideClass(Token, String),
}
