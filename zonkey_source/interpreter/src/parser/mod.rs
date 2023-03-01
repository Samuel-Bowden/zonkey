mod status;

use self::status::ParserStatus;
use crate::{
    assignment_operator::{
        BooleanAssignmentOperator, NumericAssignmentOperator, StringAssignmentOperator,
    },
    comparison::{BooleanComparision, NumericComparision, StringComparision},
    debug_information,
    err::parser::{ParserErr, ParserErrType},
    expr::{BooleanExpr, Expr, FloatExpr, IntegerExpr, NoneExpr, StringExpr},
    native_function::{
        cli_api::{CliFunctionNone, CliFunctionString},
        gui_api::GuiFunctionNone,
        NativeFunctionNone, NativeFunctionString,
    },
    operator::{NumericOperator, StringOperator},
    parser_debug,
    return_type::ReturnType,
    start::Start,
    stmt::{Stmt, ConstructionType},
    token::{Token, TokenType},
    unary_operator::{BooleanUnaryOperator, NumericUnaryOperator},
    value_type::ValueType, callable::Callable, declaration::{FunctionDeclaration, ClassDeclaration}, value::{Value, Object},
};
use indexmap::IndexMap;
use rustc_hash::FxHashMap;

pub struct Parser {
    tokens: Vec<Token>,
    value_stack: Vec<IndexMap<String, Value>>,
    integer_next_id: usize,
    float_next_id: usize,
    string_next_id: usize,
    boolean_next_id: usize,
    function_declarations: FxHashMap<String, FunctionDeclaration>,
    class_declarations: FxHashMap<String, ClassDeclaration>,
    current_function_declaration: Option<FunctionDeclaration>,
    callables: Vec<Callable>,
    error: ParserErr,
    start: Option<Start>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            value_stack: vec![],
            integer_next_id: 0,
            float_next_id: 0,
            string_next_id: 0,
            boolean_next_id: 0,
            function_declarations: FxHashMap::default(),
            class_declarations: FxHashMap::default(),
            current_function_declaration: None,
            callables: vec![],
            error: ParserErr::new(),
            start: None,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn current_token_type(&self) -> Option<&TokenType> {
        if let Some(t) = self.tokens.get(self.current) {
            Some(&t.token_type)
        } else {
            None
        }
    }

    pub fn run(mut self) -> Result<(Stmt, Vec<Callable>), ParserErr> {
        parser_debug!("Production rule path:");

        self.program();

        parser_debug!("Printing statements");

        #[cfg(debug_assertions)]
        println!("Start block: {:?}", self.start);

        if let Some(s) = self.start {
            if !self.error.had_error() {
                if let Some(stmt) = s.stmt {
                    return Ok((stmt, self.callables));
                }
            }
        } else {
            self.error.add(ParserErrType::NoStartBlock);
        }

        Err(self.error)
    }

    fn program(&mut self) {
        debug_information!("program");

        while !self.is_at_end() {
            match self.global_declaration() {
                Ok(()) => (),
                // Synchronise on both end and unwind errors in global scope
                Err(_) => loop {
                    if let Some(TokenType::Start | TokenType::Function) | None =
                        self.current_token_type()
                    {
                        break;
                    }

                    self.current += 1;
                },
            };
        }
    }

    fn global_declaration(&mut self) -> Result<(), ParserStatus> {
        debug_information!("global_declaration");

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Start,
                ..
            }) => self.start_declaration(),
            Some(Token {
                token_type: TokenType::Function,
                ..
            }) => self.function_declaration(),
            Some(Token {
                token_type: TokenType::Class,
                ..
            }) => self.class_declaration(),
            Some(unexpected_token) => {
                self.error.add(ParserErrType::UnexpectedTokenInGlobal(
                    unexpected_token.clone(),
                ));
                Err(ParserStatus::Unwind)
            }
            None => Ok(()),
        }
    }

    fn start_declaration(&mut self) -> Result<(), ParserStatus> {
        debug_information!("start_declaration");

        let start_token = self.tokens[self.current].clone();
        self.current += 1;

        // Add start value scope
        self.value_stack.push(IndexMap::new());

        let block = self.block();

        // Clean value stack after it has been parsed
        self.value_stack.clear();
        self.integer_next_id = 0;
        self.float_next_id = 0;
        self.string_next_id = 0;
        self.boolean_next_id = 0;

        if let Some(s) = &self.start {
            self.error
                .add(ParserErrType::RedefinedStart(s.token.clone(), start_token));
            return Err(ParserStatus::Unwind);
        }

        let mut start = Start {
            stmt: None,
            token: start_token,
        };

        match block {
            Ok(block) => {
                start.stmt = Some(block);
                self.start = Some(start);
                Ok(())
            }
            Err(e) => {
                self.start = Some(start);
                Err(e)
            }
        }
    }

    fn function_declaration(&mut self) -> Result<(), ParserStatus> {
        debug_information!("function_declaration");

        // First stage - parse function
        let function_token_pos = self.current;
        self.current += 1;

        let function_name = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
            t => {
                self.error
                    .add(ParserErrType::FunctionDeclarationExpectedName(
                        self.tokens[function_token_pos].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error
                    .add(ParserErrType::FunctionDeclarationExpectedLeftParen(
                        self.tokens[self.current - 1].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        // Get function parameters
        let mut parameters = vec![];

        match self.current_token_type() {
            Some(TokenType::RightParen) => {
                self.current += 1;
            }
            _ => loop {
                let parameter_data_type = match self.data_type() {
                    Ok(data_type) => data_type,
                    Err(t) => {
                        self.error
                            .add(ParserErrType::FunctionDeclarationExpectedParameterType(
                                self.tokens[self.current - 1].clone(),
                                t,
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };
                self.current += 1;

                let parameter_name = match self.tokens.get(self.current) {
                    Some(Token {
                        token_type: TokenType::Identifier(name),
                        ..
                    }) => name.clone(),
                    t => {
                        self.error
                            .add(ParserErrType::FunctionDeclarationExpectedParameterName(
                                self.tokens[function_token_pos].clone(),
                                t.cloned(),
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };
                self.current += 1;

                parameters.push((parameter_data_type, parameter_name));

                match self.tokens.get(self.current) {
                    Some(Token {
                        token_type: TokenType::Comma,
                        ..
                    }) => {
                        self.current += 1;
                        continue;
                    }
                    Some(Token {
                        token_type: TokenType::RightParen,
                        ..
                    }) => {
                        self.current += 1;
                        break;
                    }
                    t => {
                        self.error.add(
                            ParserErrType::FunctionDeclarationExpectedCommaOrRightParen(
                                self.tokens[function_token_pos].clone(),
                                t.cloned(),
                            ),
                        );
                        return Err(ParserStatus::Unwind);
                    }
                };
            },
        }

        // Get return type if present
        let return_data_type = if let Some(TokenType::Arrow) = self.current_token_type() {
            self.current += 1;

            match self.return_type() {
                Ok(return_type) => {
                    self.current += 1;
                    return_type
                }
                Err(t) => {
                    self.error
                        .add(ParserErrType::FunctionDeclarationExpectedReturnType(
                            self.tokens[self.current - 1].clone(),
                            t,
                        ));
                    return Err(ParserStatus::Unwind);
                }
            }
        } else {
            ReturnType::None
        };

        // Second stage - parse function body
        // Add parameters to the first value scope of function body
        let mut function_scope = IndexMap::new();
        for (value_type, name) in &parameters {
            self.add_function_parameter(value_type, name, &mut function_scope);
        }

        self.value_stack.push(function_scope);

        let function_declaration = FunctionDeclaration {
            id: self.callables.len(),
            parameters,
            return_data_type,
        };

        self.function_declarations
            .insert(function_name, function_declaration.clone());

        self.current_function_declaration = Some(function_declaration);

        // Parse the function block
        let block = self.block()?;

        // Clean value stack after it has been parsed
        self.value_stack.clear();
        self.integer_next_id = 0;
        self.float_next_id = 0;
        self.string_next_id = 0;
        self.boolean_next_id = 0;

        self.current_function_declaration = None;

        // Finally add function to callables
        self.callables.push(Callable { start: block });

        Ok(())
    }

    fn class_declaration(&mut self) -> Result<(), ParserStatus> {
        debug_information!("class_declaration");

        let class_token_pos = self.current;
        self.current += 1;

        let class_name = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
            t => {
                self.error.add(ParserErrType::ClassDeclarationExpectedName(
                    self.tokens[class_token_pos].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let open_brace_pos = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftBrace,
                ..
            }) => self.current,
            t => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectedLeftBrace(
                        self.tokens[self.current - 1].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let mut properties = FxHashMap::default();

        while let Ok(dt) = self.data_type() {
            self.current += 1;

            match self.tokens.get(self.current) {
                Some(Token {
                    token_type: TokenType::Identifier(name),
                    ..
                }) => properties.insert(name.clone(), dt),
                t => {
                    self.error
                        .add(ParserErrType::ClassDeclarationExpectedPropertyName(
                            self.tokens[self.current - 1].clone(),
                            t.cloned(),
                        ));
                    return Err(ParserStatus::Unwind);
                }
            };
            self.current += 1;

            match self.tokens.get(self.current) {
                Some(Token {
                    token_type: TokenType::SemiColon,
                    ..
                }) => (),
                t => {
                    self.error
                        .add(ParserErrType::ClassDeclarationUnterminatedProperty(
                            self.tokens[self.current - 1].clone(),
                            t.cloned(),
                        ));
                    return Err(ParserStatus::Unwind);
                }
            };
            self.current += 1;
        }

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::RightBrace,
                ..
            }) => (),
            _ => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectedRightBrace(
                        self.tokens[open_brace_pos].clone(),
                        self.tokens[self.current - 1].clone(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let class_declaration = ClassDeclaration { 
            properties,
        };

        self.class_declarations
            .insert(class_name, class_declaration);

        Ok(())
    }

    fn local_declaration(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("local_declaration");

        match self.current_token_type() {
            Some(TokenType::Let) => self.terminated_variable_declaration(),
            _ => self.statement(),
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("statement");

        match self.current_token_type() {
            Some(TokenType::LeftBrace) => self.block(),
            Some(TokenType::If) => {
                self.current += 1;
                self.if_statement()
            }
            Some(TokenType::While) => {
                self.current += 1;
                self.while_statement()
            }
            Some(TokenType::Loop) => {
                self.current += 1;
                self.loop_statement()
            }
            Some(TokenType::For) => {
                self.current += 1;
                self.for_statement()
            }
            _ => Ok(self.terminated_statement()?),
        }
    }

    fn terminated_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("terminated_statement");

        let statement = match self.current_token_type() {
            Some(TokenType::Return) => self.return_statement()?,
            Some(TokenType::Break) => {
                self.current += 1;
                Stmt::Break
            }
            Some(TokenType::Continue) => {
                self.current += 1;
                Stmt::Continue
            }
            _ => self.expression_statement()?,
        };

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::SemiColon,
                ..
            }) => {
                self.current += 1;
                return Ok(statement);
            }
            t => self.error.add(ParserErrType::UnterminatedStatement(
                self.tokens[self.current - 1].clone(),
                t.cloned(),
            )),
        }

        Err(ParserStatus::Unwind)
    }

    fn return_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("return_statement");

        let return_token_position = self.current;
        self.current += 1;

        let function_ret_type = if let Some(function) = &self.current_function_declaration {
            function.return_data_type.clone()
        } else {
            self.error.add(ParserErrType::StartCannotReturn(
                self.tokens[return_token_position].clone(),
            ));
            return Err(ParserStatus::Unwind);
        };

        let expression = match self.current_token_type() {
            Some(TokenType::SemiColon) => None,
            _ => Some(self.expression()?),
        };

        Ok(Stmt::Return(match (function_ret_type, expression) {
            (ReturnType::Integer, Some(Expr::Integer(expr))) => Some(Expr::Integer(expr)),
            (ReturnType::Float, Some(Expr::Float(expr))) => Some(Expr::Float(expr)),
            (ReturnType::String, Some(Expr::String(expr))) => Some(Expr::String(expr)),
            (ReturnType::Boolean, Some(Expr::Boolean(expr))) => Some(Expr::Boolean(expr)),
            (ReturnType::None, Some(Expr::None(expr))) => Some(Expr::None(expr)),
            (ReturnType::None, None) => None,
            (ret_type, expr) => {
                let expr_type = if let Some(expr) = expr {
                    self.expr_type(&expr)
                } else {
                    ReturnType::None
                };

                self.error.add(
                    ParserErrType::FunctionDeclarationInvalidReturnExpressionType(
                        self.tokens[return_token_position].clone(),
                        ret_type.clone(),
                        expr_type,
                    ),
                );

                return Err(ParserStatus::Unwind);
            }
        }))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("if_statement");

        let left_paren = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::IfExpectedLeftParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let expression = self.expression()?;

        let right_paren = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::RightParen,
                end,
                ..
            }) => *end,
            t => {
                self.error.add(ParserErrType::IfExpectedRightParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };

        self.current += 1;

        let expression = if let Expr::Boolean(expr) = expression {
            expr
        } else {
            self.error.add(ParserErrType::IfConditionNotBool(
                left_paren + 1,
                right_paren - 1,
            ));
            // Place dummy expression to continue parsing rest for errors
            BooleanExpr::Literal(false)
        };

        let true_branch = Box::new(self.block()?);

        let false_branch = match self.current_token_type() {
            Some(TokenType::Else) => {
                self.current += 1;

                Some(Box::new(self.statement()?))
            }
            _ => None,
        };

        Ok(Stmt::If(expression, true_branch, false_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("while_statement");

        let left_paren = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::WhileExpectedLeftParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };

        self.current += 1;

        let expression = self.expression()?;

        let right_paren = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::RightParen,
                end,
                ..
            }) => *end,
            t => {
                self.error.add(ParserErrType::WhileExpectedRightParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };

        self.current += 1;

        let expression = if let Expr::Boolean(expr) = expression {
            expr
        } else {
            self.error.add(ParserErrType::WhileConditionNotBool(
                left_paren + 1,
                right_paren - 1,
            ));
            // Place dummy expression to continue parsing rest for errors
            BooleanExpr::Literal(false)
        };

        let block = Box::new(self.block()?);

        Ok(Stmt::While(expression, block))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("for_statement");

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftParen,
                ..
            }) => {
                self.current += 1;
            }
            t => {
                self.error.add(ParserErrType::ForExpectedLeftParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };

        self.value_stack.push(IndexMap::new());
        let integer_point = self.integer_next_id;
        let float_point = self.float_next_id;
        let string_point = self.string_next_id;
        let boolean_point = self.boolean_next_id;

        // Abort parsing when there are errors parsing the parameters, as a block has been
        // added and it will be very difficult to synchronise.
        let initialiser_statement = match self.variable_declaration() {
            Ok(is) => is,
            Err(_) => return Err(ParserStatus::End),
        };

        let test_statement_start = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Comma,
                end,
                ..
            }) => *end,
            t => {
                self.error.add(ParserErrType::ForExpectedComma1(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };
        self.current += 1;

        let test_statement = match self.expression() {
            Ok(ts) => ts,
            Err(_) => return Err(ParserStatus::End),
        };

        let test_statement_end = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Comma,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::ForExpectedComma2(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };
        self.current += 1;

        let test_statement = if let Expr::Boolean(expr) = test_statement {
            expr
        } else {
            self.error.add(ParserErrType::ForConditionNotBool(
                test_statement_start,
                test_statement_end,
            ));
            // Place dummy expression to continue parsing rest for errors
            BooleanExpr::Literal(false)
        };

        let update_statement = match self.expression_statement() {
            Ok(us) => us,
            Err(_) => return Err(ParserStatus::End),
        };

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::RightParen,
                ..
            }) => {
                self.current += 1;
            }
            t => {
                self.error.add(ParserErrType::ForExpectedRightParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };

        let mut block = self.block()?;

        if let Stmt::Block(b, _) = &mut block {
            b.push(update_statement);
        }

        self.value_stack.pop();

        self.integer_next_id = integer_point;
        self.float_next_id = float_point;
        self.string_next_id = string_point;
        self.boolean_next_id = boolean_point;

        Ok(Stmt::Block(
            vec![
                initialiser_statement,
                Stmt::While(test_statement, Box::new(block)),
            ],
            (integer_point, float_point, string_point, boolean_point),
        ))
    }

    fn loop_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("loop_statement");

        let block = Box::new(self.block()?);

        Ok(Stmt::Loop(block))
    }

    fn block(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("block");

        let open_brace_pos = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftBrace,
                ..
            }) => self.current,
            t => {
                self.error.add(ParserErrType::BlockExpectedLeftBrace(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let mut statements = vec![];
        self.value_stack.push(IndexMap::new());

        let integer_point = self.integer_next_id;
        let float_point = self.float_next_id;
        let string_point = self.string_next_id;
        let boolean_point = self.boolean_next_id;

        loop {
            match self.current_token_type() {
                Some(TokenType::RightBrace) => {
                    self.current += 1;
                    self.value_stack.pop();

                    self.integer_next_id = integer_point;
                    self.float_next_id = float_point;
                    self.string_next_id = string_point;
                    self.boolean_next_id = boolean_point;

                    return Ok(Stmt::Block(
                        statements,
                        (integer_point, float_point, string_point, boolean_point),
                    ));
                }
                Some(_) => {
                    match self.local_declaration() {
                        Ok(s) => statements.push(s),
                        Err(ParserStatus::Unwind) => {
                            // Best effort to synchronise on the end or start of statements
                            let mut braces_seen = 0;

                            loop {
                                match self.current_token_type() {
                                    // Statement end
                                    Some(TokenType::SemiColon) => {
                                        if braces_seen == 0 {
                                            self.current += 1;
                                            break;
                                        }
                                    }
                                    // Statement start
                                    Some(
                                        TokenType::Let
                                        | TokenType::Identifier(_)
                                        | TokenType::If
                                        | TokenType::For
                                        | TokenType::Return
                                        | TokenType::Loop
                                        | TokenType::While,
                                    ) => {
                                        if braces_seen == 0 {
                                            break;
                                        }
                                    }
                                    Some(TokenType::RightBrace) => {
                                        if braces_seen == 0 {
                                            break;
                                        } else {
                                            braces_seen -= 1;
                                        }
                                    }
                                    Some(TokenType::LeftBrace) => {
                                        braces_seen += 1;
                                    }
                                    _ => (),
                                }

                                self.current += 1;
                            }
                        }
                        Err(ParserStatus::End) => return Err(ParserStatus::End),
                    }
                }
                None => {
                    self.error.add(ParserErrType::BlockExpectedRightBrace(
                        self.tokens[open_brace_pos].clone(),
                        self.tokens[self.current - 1].clone(),
                    ));

                    return Err(ParserStatus::End);
                }
            }
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("expression_statement");

        let expr = self.expression()?;

        match self.current_token_type() {
            Some(
                TokenType::Equal
                | TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual,
            ) => {
                let assignment_operator = self.current;
                self.current += 1;

                let value = self.expression()?;

                match (expr, value) {
                    (Expr::Integer(IntegerExpr::Variable(id)), Expr::Integer(val)) => {
                        Ok(Stmt::IntegerVariableAssignment(
                            id,
                            val,
                            match self.tokens[assignment_operator].token_type {
                                TokenType::Equal => NumericAssignmentOperator::Equal,
                                TokenType::PlusEqual => NumericAssignmentOperator::PlusEqual,
                                TokenType::MinusEqual => NumericAssignmentOperator::MinusEqual,
                                TokenType::StarEqual => NumericAssignmentOperator::StarEqual,
                                _ => NumericAssignmentOperator::SlashEqual,
                            },
                        ))
                    }
                    (Expr::Float(FloatExpr::Variable(id)), Expr::Float(val)) => {
                        Ok(Stmt::FloatVariableAssignment(
                            id,
                            val,
                            match self.tokens[assignment_operator].token_type {
                                TokenType::Equal => NumericAssignmentOperator::Equal,
                                TokenType::PlusEqual => NumericAssignmentOperator::PlusEqual,
                                TokenType::MinusEqual => NumericAssignmentOperator::MinusEqual,
                                TokenType::StarEqual => NumericAssignmentOperator::StarEqual,
                                _ => NumericAssignmentOperator::SlashEqual,
                            },
                        ))
                    }
                    (Expr::String(StringExpr::Variable(id)), Expr::String(val)) => {
                        Ok(Stmt::StringVariableAssignment(
                            id,
                            val,
                            match self.tokens[assignment_operator].token_type {
                                TokenType::Equal => StringAssignmentOperator::Equal,
                                TokenType::PlusEqual => StringAssignmentOperator::PlusEqual,
                                _ => {
                                    self.error.add(ParserErrType::InvalidAssignmentOperator(
                                        self.tokens[assignment_operator].clone(),
                                        ValueType::String,
                                    ));
                                    return Err(ParserStatus::Unwind);
                                }
                            },
                        ))
                    }
                    (Expr::Boolean(BooleanExpr::Variable(id)), Expr::Boolean(val)) => {
                        Ok(Stmt::BooleanVariableAssignment(
                            id,
                            val,
                            match self.tokens[assignment_operator].token_type {
                                TokenType::Equal => BooleanAssignmentOperator::Equal,
                                _ => {
                                    self.error.add(ParserErrType::InvalidAssignmentOperator(
                                        self.tokens[assignment_operator].clone(),
                                        ValueType::Boolean,
                                    ));
                                    return Err(ParserStatus::Unwind);
                                }
                            },
                        ))
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        self.error
                            .add(ParserErrType::UnmatchingTypesAssignmentOperatator(
                                self.tokens[assignment_operator].clone(),
                                left,
                                right,
                            ));

                        return Err(ParserStatus::Unwind);
                    }
                }
            }
            _ => Ok(Stmt::Expression(expr)),
        }
    }

    fn variable_declaration(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("variable_declaration");
        self.current += 1;

        let name = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
            t => {
                self.error
                    .add(ParserErrType::VariableDeclarationExpectedName(
                        self.tokens[self.current - 1].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        if let Some(_) = self.value_stack.last().unwrap().get(&name) {
            self.error
                .add(ParserErrType::VariableDeclarationAlreadyDeclared(
                    self.tokens[self.current - 1].clone(),
                    name,
                ));
            return Err(ParserStatus::Unwind);
        }

        let equal_pos = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Equal,
                ..
            }) => self.current,
            t => {
                self.error
                    .add(ParserErrType::VariableDeclarationExpectedEqual(
                        self.tokens[self.current - 1].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        if let Some(TokenType::New) = self.current_token_type() {
            self.current += 1;
            
            match self.current_token_type().cloned() {
                Some(TokenType::Identifier(class_name)) => {
                    self.current += 1;
                    let (object, types) = self.create_object(class_name)?;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name, Value::Object(object));

                    Ok(Stmt::ClassVariableDeclaration(types))
                }
                _ => {
                    panic!("Expected identifier")
                }
            }
        } else {
            let expr = self.expression()?;

            match expr {
                Expr::Integer(val) => {
                    let id = self.integer_next_id;
                    self.integer_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), Value::Integer(id));
                    Ok(Stmt::IntegerVariableDeclaration(val))
                }
                Expr::Float(val) => {
                    let id = self.float_next_id;
                    self.float_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), Value::Float(id));
                    Ok(Stmt::FloatVariableDeclaration(val))
                }
                Expr::String(val) => {
                    let id = self.string_next_id;
                    self.string_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), Value::String(id));
                    Ok(Stmt::StringVariableDeclaration(val))
                }
                Expr::Boolean(val) => {
                    let id = self.boolean_next_id;
                    self.boolean_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), Value::Boolean(id));
                    Ok(Stmt::BooleanVariableDeclaration(val))
                }
                Expr::None(_) => {
                    self.error
                        .add(ParserErrType::VariableDeclarationExprEvalNone(
                            self.tokens[equal_pos].end,
                            self.tokens[self.current].end,
                        ));
                    Err(ParserStatus::Unwind)
                }
                Expr::Object(..) => {
                    todo!()
                }
            }
        }
    }

    fn terminated_variable_declaration(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("terminated_variable_declaration");

        let variable_declaration = self.variable_declaration()?;

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::SemiColon,
                ..
            }) => {
                self.current += 1;
                return Ok(variable_declaration);
            }
            t => self.error.add(ParserErrType::UnterminatedStatement(
                self.tokens[self.current - 1].clone(),
                t.cloned(),
            )),
        }

        Err(ParserStatus::Unwind)
    }

    fn expression(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("expression");
        self.cast()
    }

    fn cast(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("cast");

        match self.current_token_type() {
            Some(TokenType::IntegerType) => {
                let integer_type_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match expression {
                    Expr::Integer(expr) => {
                        self.error.add(ParserErrType::CastPointless(
                            self.tokens[integer_type_pos].clone(),
                            ReturnType::Integer,
                        ));

                        Ok(Expr::Integer(expr))
                    }
                    Expr::Float(expr) => Ok(Expr::Integer(IntegerExpr::FloatCast(Box::new(expr)))),
                    Expr::Boolean(expr) => {
                        Ok(Expr::Integer(IntegerExpr::BooleanCast(Box::new(expr))))
                    }
                    Expr::String(expr) => {
                        Ok(Expr::Integer(IntegerExpr::StringCast(Box::new(expr))))
                    }
                    expr => {
                        self.error.add(ParserErrType::CastNotPossible(
                            self.tokens[integer_type_pos].clone(),
                            ReturnType::Integer,
                            self.expr_type(&expr),
                        ));

                        Err(ParserStatus::Unwind)
                    }
                }
            }
            Some(TokenType::FloatType) => {
                let float_type_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match expression {
                    Expr::Float(expr) => {
                        self.error.add(ParserErrType::CastPointless(
                            self.tokens[float_type_pos].clone(),
                            ReturnType::Float,
                        ));

                        Ok(Expr::Float(expr))
                    }
                    Expr::Integer(expr) => Ok(Expr::Float(FloatExpr::IntegerCast(Box::new(expr)))),
                    Expr::Boolean(expr) => Ok(Expr::Float(FloatExpr::BooleanCast(Box::new(expr)))),
                    Expr::String(expr) => Ok(Expr::Float(FloatExpr::StringCast(Box::new(expr)))),
                    expr => {
                        self.error.add(ParserErrType::CastNotPossible(
                            self.tokens[float_type_pos].clone(),
                            ReturnType::Float,
                            self.expr_type(&expr),
                        ));

                        Err(ParserStatus::Unwind)
                    }
                }
            }
            Some(TokenType::StringType) => {
                let string_type_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match expression {
                    Expr::String(expr) => {
                        self.error.add(ParserErrType::CastPointless(
                            self.tokens[string_type_pos].clone(),
                            ReturnType::String,
                        ));

                        Ok(Expr::String(expr))
                    }
                    Expr::Integer(expr) => {
                        Ok(Expr::String(StringExpr::IntegerCast(Box::new(expr))))
                    }
                    Expr::Float(expr) => Ok(Expr::String(StringExpr::FloatCast(Box::new(expr)))),
                    Expr::Boolean(expr) => {
                        Ok(Expr::String(StringExpr::BooleanCast(Box::new(expr))))
                    }
                    expr => {
                        self.error.add(ParserErrType::CastNotPossible(
                            self.tokens[string_type_pos].clone(),
                            ReturnType::String,
                            self.expr_type(&expr),
                        ));

                        Err(ParserStatus::Unwind)
                    }
                }
            }
            Some(TokenType::BooleanType) => {
                let boolean_type_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match expression {
                    Expr::Boolean(expr) => {
                        self.error.add(ParserErrType::CastPointless(
                            self.tokens[boolean_type_pos].clone(),
                            ReturnType::Boolean,
                        ));

                        Ok(Expr::Boolean(expr))
                    }
                    Expr::Integer(expr) => {
                        Ok(Expr::Boolean(BooleanExpr::IntegerCast(Box::new(expr))))
                    }
                    Expr::Float(expr) => Ok(Expr::Boolean(BooleanExpr::FloatCast(Box::new(expr)))),
                    Expr::String(expr) => {
                        Ok(Expr::Boolean(BooleanExpr::StringCast(Box::new(expr))))
                    }
                    expr => {
                        self.error.add(ParserErrType::CastNotPossible(
                            self.tokens[boolean_type_pos].clone(),
                            ReturnType::Boolean,
                            self.expr_type(&expr),
                        ));

                        Err(ParserStatus::Unwind)
                    }
                }
            }
            _ => self.or(),
        }
    }

    fn or(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("or");

        let mut left = self.and()?;

        loop {
            if let Some(TokenType::Or) = self.current_token_type() {
                let or_token_pos = self.current;
                self.current += 1;

                let right = self.and()?;

                match (left, right) {
                    (Expr::Boolean(left_inside), Expr::Boolean(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::BooleanBinary {
                            left: Box::new(left_inside),
                            comparator: BooleanComparision::Or,
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::ComparisionInvalidForType(
                                self.tokens[or_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                                self.tokens[or_token_pos].clone(),
                                left,
                                right,
                            ));
                        }

                        return Err(ParserStatus::Unwind);
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("and");

        let mut left = self.equality()?;

        loop {
            if let Some(TokenType::And) = self.current_token_type() {
                let and_token_pos = self.current;
                self.current += 1;

                let right = self.equality()?;

                match (left, right) {
                    (Expr::Boolean(left_inside), Expr::Boolean(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::BooleanBinary {
                            left: Box::new(left_inside),
                            comparator: BooleanComparision::And,
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::ComparisionInvalidForType(
                                self.tokens[and_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                                self.tokens[and_token_pos].clone(),
                                left,
                                right,
                            ));
                        }

                        return Err(ParserStatus::Unwind);
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("equality");

        let mut left = self.comparision()?;

        loop {
            if let Some(TokenType::EqualEqual | TokenType::BangEqual) = self.current_token_type() {
                let comparator_token_pos = self.current;
                self.current += 1;

                let right = self.comparision()?;

                let comparator_type = &self.tokens[comparator_token_pos].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::IntegerBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => NumericComparision::Equal,
                                _ => NumericComparision::Inequal,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::FloatBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => NumericComparision::Equal,
                                _ => NumericComparision::Inequal,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(left_inside), Expr::String(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::StringBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => StringComparision::Equal,
                                _ => StringComparision::Inequal,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Boolean(left_inside), Expr::Boolean(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::BooleanBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => BooleanComparision::Equal,
                                _ => BooleanComparision::Inequal,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::ComparisionInvalidForType(
                                self.tokens[comparator_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                                self.tokens[comparator_token_pos].clone(),
                                left,
                                right,
                            ));
                        }

                        return Err(ParserStatus::Unwind);
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn comparision(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("comparison");

        let mut left = self.addsub()?;

        loop {
            if let Some(
                TokenType::MoreEqual | TokenType::LessEqual | TokenType::Less | TokenType::More,
            ) = self.current_token_type()
            {
                let comparator_token_pos = self.current;
                self.current += 1;

                let right = self.addsub()?;

                let comparator_type = &self.tokens[comparator_token_pos].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::IntegerBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::MoreEqual => NumericComparision::MoreEqual,
                                TokenType::LessEqual => NumericComparision::LessEqual,
                                TokenType::More => NumericComparision::More,
                                _ => NumericComparision::Less,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::FloatBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::MoreEqual => NumericComparision::MoreEqual,
                                TokenType::LessEqual => NumericComparision::LessEqual,
                                TokenType::More => NumericComparision::More,
                                _ => NumericComparision::Less,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::ComparisionInvalidForType(
                                self.tokens[comparator_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                                self.tokens[comparator_token_pos].clone(),
                                left,
                                right,
                            ));
                        }

                        return Err(ParserStatus::Unwind);
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn addsub(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("addsub");

        let mut left = self.multdiv()?;

        loop {
            if let Some(TokenType::Minus | TokenType::Plus) = self.current_token_type() {
                let operator_token_pos = self.current;
                self.current += 1;

                let right = self.multdiv()?;

                let operator_type = &self.tokens[operator_token_pos].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Integer(IntegerExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Plus => NumericOperator::Add,
                                _ => NumericOperator::Subtract,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Float(FloatExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Plus => NumericOperator::Add,
                                _ => NumericOperator::Subtract,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(left_inside), Expr::String(right_inside)) => {
                        left = Expr::String(StringExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Plus => StringOperator::Add,
                                _ => {
                                    self.error.add(ParserErrType::OperatorInvalidForType(
                                        self.tokens[operator_token_pos].clone(),
                                        ReturnType::String,
                                    ));

                                    return Err(ParserStatus::Unwind);
                                }
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::OperatorInvalidForType(
                                self.tokens[operator_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::OperatorUnmatchingTypes(
                                self.tokens[operator_token_pos].clone(),
                                left,
                                right,
                            ));
                        }

                        return Err(ParserStatus::Unwind);
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn multdiv(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("multdiv");

        let mut left = self.unary()?;

        loop {
            if let Some(TokenType::Star | TokenType::Slash) = self.current_token_type() {
                let operator_token_pos = self.current;
                self.current += 1;

                let right = self.unary()?;

                let operator_type = &self.tokens[operator_token_pos].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Integer(IntegerExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Star => NumericOperator::Multiply,
                                _ => NumericOperator::Divide,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Float(FloatExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Star => NumericOperator::Multiply,
                                _ => NumericOperator::Divide,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::OperatorInvalidForType(
                                self.tokens[operator_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::OperatorUnmatchingTypes(
                                self.tokens[operator_token_pos].clone(),
                                left,
                                right,
                            ));
                        }

                        return Err(ParserStatus::Unwind);
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, ParserStatus> {
        if let Some(TokenType::Minus | TokenType::Bang) = self.current_token_type() {
            let operator_pos = self.current;
            self.current += 1;

            let unary_expr = self.unary()?;

            let operator_type = &self.tokens[operator_pos].token_type;

            match (operator_type, unary_expr) {
                (TokenType::Minus, Expr::Integer(expr)) => Ok(Expr::Integer(IntegerExpr::Unary(
                    NumericUnaryOperator::Minus,
                    Box::new(expr),
                ))),
                (TokenType::Minus, Expr::Float(expr)) => Ok(Expr::Float(FloatExpr::Unary(
                    NumericUnaryOperator::Minus,
                    Box::new(expr),
                ))),
                (TokenType::Bang, Expr::Boolean(expr)) => Ok(Expr::Boolean(BooleanExpr::Unary(
                    BooleanUnaryOperator::Bang,
                    Box::new(expr),
                ))),
                (_, expr) => {
                    let expr_type = self.expr_type(&expr);

                    self.error.add(ParserErrType::UnaryOperatorInvalidForType(
                        self.tokens[operator_pos].clone(),
                        expr_type,
                    ));

                    Err(ParserStatus::Unwind)
                }
            }
        } else {
            self.literal()
        }
    }

    fn literal(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("literal");

        match self.current_token_type().cloned() {
            Some(TokenType::Integer(val)) => {
                self.current += 1;
                Ok(Expr::Integer(IntegerExpr::Literal(val)))
            }
            Some(TokenType::Float(val)) => {
                self.current += 1;
                Ok(Expr::Float(FloatExpr::Literal(val)))
            }
            Some(TokenType::String(val)) => {
                self.current += 1;
                Ok(Expr::String(StringExpr::Literal(val)))
            }
            Some(TokenType::Boolean(val)) => {
                self.current += 1;
                Ok(Expr::Boolean(BooleanExpr::Literal(val)))
            }
            Some(TokenType::LeftParen) => {
                let left_paren_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match self.current_token_type() {
                    Some(TokenType::RightParen) => {
                        self.current += 1;
                        Ok(expression)
                    }
                    _ => {
                        self.error.add(ParserErrType::GroupingExpectedRightParen(
                            self.tokens[left_paren_pos].clone(),
                            self.tokens.get(self.current).cloned(),
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                }
            }
            Some(TokenType::Identifier(mut name)) => {
                self.current += 1;

                match self.current_token_type() {
                    Some(TokenType::LeftParen) => {
                        self.call(&name, None, self.current)
                    }
                    Some(TokenType::Colon) => {
                        self.current += 1;

                        match self.current_token_type() {
                            Some(TokenType::Identifier(second_name)) => {
                                let second_name = second_name.clone();
                                self.current += 1;

                                match self.current_token_type() {
                                    Some(TokenType::LeftParen) => {
                                        self.call(&second_name, Some(name.clone()), self.current - 1)
                                    }
                                    _ => {
                                        self.error.add(ParserErrType::ModuleExpectedLeftParen(
                                            self.tokens[self.current - 1].clone(),
                                            self.tokens.get(self.current).cloned(),
                                        ));
                                        Err(ParserStatus::Unwind)
                                    }
                                }
                            }
                            _ => {
                                self.error.add(ParserErrType::ModuleExpectedIdentifier(
                                    self.tokens[self.current - 2].clone(),
                                    self.tokens.get(self.current - 1).cloned(),
                                ));
                                Err(ParserStatus::Unwind)
                            }
                        }
                    }
                    Some(TokenType::Dot) => {
                        let mut object = match self.find_value(&name) {
                            Some(Value::Object(obj)) => {
                                obj
                            }
                            _ => {
                                panic!("Value {name} is not an object");
                            }
                        };

                        loop {
                            self.current += 1;

                            name = if let Some(TokenType::Identifier(name)) = self.current_token_type() {
                                name.clone()
                            } else {
                                panic!("Expected identifier for property name");
                            };

                            self.current += 1;

                            match (object.properties.get(&name), self.current_token_type()) {
                                (Some(Value::Object(obj)), Some(TokenType::Dot)) => {
                                    object = obj.clone();
                                    continue;
                                }
                                (Some(v), _) => {
                                    return Ok(self.get_variable_expr(v.clone(), name))
                                }
                                (_, Some(TokenType::Dot)) => {
                                    panic!("Value {name} is not an object");
                                }
                                (None, _) => {
                                    panic!("Property does not exist");
                                }
                            }
                        }
                    }
                    _ => {
                        match self.find_value(&name) {
                            Some(value) => Ok(self.get_variable_expr(value, name)),
                            None => {
                                panic!("Value not found");
                            }
                        }
                    }
                }
            }
            _ => {
                self.error.add(ParserErrType::ExpectedLiteralVariableCall(
                    self.tokens[self.current - 1].clone(),
                    self.tokens.get(self.current).cloned(),
                ));
                Err(ParserStatus::Unwind)
            }
        }
    }

    fn call(
        &mut self,
        name: &str,
        module: Option<String>,
        token_pos: usize,
    ) -> Result<Expr, ParserStatus> {
        debug_information!("call");
        self.current += 1;

        let mut arguments = vec![];

        match self.current_token_type() {
            Some(TokenType::RightParen) => {
                self.current += 1;
            }
            _ => loop {
                let argument = self.expression()?;

                arguments.push(argument);

                match self.tokens.get(self.current) {
                    Some(Token {
                        token_type: TokenType::Comma,
                        ..
                    }) => {
                        self.current += 1;
                        continue;
                    }
                    Some(Token {
                        token_type: TokenType::RightParen,
                        ..
                    }) => {
                        self.current += 1;
                        break;
                    }
                    t => {
                        self.error.add(ParserErrType::CallExpectedCommaOrRightParen(
                            self.tokens[self.current - 1].clone(),
                            t.cloned(),
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                };
            },
        }

        if let Some(module) = module {
            #[allow(dead_code)]
            enum InternalType {
                Integer,
                Float,
                Boolean,
                String,
                Printable,
            }

            let parameters = match (module.as_str(), name) {
                ("cli", "println") => vec![InternalType::Printable],
                ("cli", "print") => vec![InternalType::Printable],
                ("cli", "prompt") => vec![InternalType::String],
                ("gui", "add_heading") => vec![InternalType::String],
                ("gui", "add_paragraph") => vec![InternalType::String],
                ("cli" | "gui", _) => {
                    self.error.add(ParserErrType::CallModuleFunctionNotFound(
                        self.tokens[token_pos].clone(),
                        name.to_string(),
                        module,
                    ));
                    return Err(ParserStatus::Unwind);
                }
                _ => {
                    self.error.add(ParserErrType::CallModuleNotFound(
                        self.tokens[token_pos - 2].clone(),
                        module,
                    ));
                    return Err(ParserStatus::Unwind);
                }
            };

            if arguments.len() != parameters.len() {
                self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                    self.tokens[token_pos].clone(),
                    arguments.len(),
                    parameters.len(),
                    name.to_string(),
                ));
                return Err(ParserStatus::Unwind);
            }

            let mut argument_error = false;

            for i in 0..arguments.len() {
                match (&arguments[i], &parameters[i]) {
                    (Expr::Integer(_), InternalType::Integer) => (),
                    (Expr::Float(_), InternalType::Float) => (),
                    (Expr::String(_), InternalType::String) => (),
                    (Expr::Boolean(_), InternalType::Boolean) => (),
                    (
                        Expr::Integer(_) | Expr::Float(_) | Expr::String(_) | Expr::Boolean(_),
                        InternalType::Printable,
                    ) => (),
                    (expr, _) => {
                        let expr_type = self.expr_type(expr);

                        self.error.add(ParserErrType::CallArgumentIncorrectType(
                            self.tokens[token_pos].clone(),
                            i,
                            expr_type,
                            name.to_string(),
                        ));

                        argument_error = true;
                    }
                }
            }

            if argument_error {
                return Err(ParserStatus::Unwind);
            }

            return Ok(match (module.as_str(), name) {
                ("cli", "println") => Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Cli(
                    match arguments.pop() {
                        Some(Expr::Integer(arg)) => {
                            CliFunctionNone::PrintLineInteger(Box::new(arg))
                        }
                        Some(Expr::Float(arg)) => CliFunctionNone::PrintLineFloat(Box::new(arg)),
                        Some(Expr::String(arg)) => CliFunctionNone::PrintLineString(Box::new(arg)),
                        Some(Expr::Boolean(arg)) => {
                            CliFunctionNone::PrintLineBoolean(Box::new(arg))
                        }
                        _ => unreachable!(),
                    },
                ))),
                ("cli", "print") => Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Cli(
                    match arguments.pop() {
                        Some(Expr::Integer(arg)) => CliFunctionNone::PrintInteger(Box::new(arg)),
                        Some(Expr::Float(arg)) => CliFunctionNone::PrintFloat(Box::new(arg)),
                        Some(Expr::String(arg)) => CliFunctionNone::PrintString(Box::new(arg)),
                        Some(Expr::Boolean(arg)) => CliFunctionNone::PrintBoolean(Box::new(arg)),
                        _ => unreachable!(),
                    },
                ))),
                ("cli", "prompt") => match arguments.pop() {
                    Some(Expr::String(argument)) => {
                        return Ok(Expr::String(StringExpr::NativeCall(
                            NativeFunctionString::Cli(CliFunctionString::Prompt(Box::new(
                                argument,
                            ))),
                        )));
                    }
                    _ => unreachable!(),
                },
                ("gui", "add_heading") => match arguments.pop() {
                    Some(Expr::String(value)) => {
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Gui(
                            GuiFunctionNone::AddHeading(Box::new(value)),
                        ))));
                    }
                    _ => unreachable!(),
                },
                ("gui", "add_paragraph") => match arguments.pop() {
                    Some(Expr::String(value)) => {
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Gui(
                            GuiFunctionNone::AddParagraph(Box::new(value)),
                        ))));
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            });
        }

        //Must be a zonkey call
        if let Some(call) = self.function_declarations.get(name) {
            if arguments.len() != call.parameters.len() {
                self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                    self.tokens[token_pos - 1].clone(),
                    arguments.len(),
                    call.parameters.len(),
                    name.to_string(),
                ));
                return Err(ParserStatus::Unwind);
            }

            // Check arguments evaluate to the same type as parameters
            for i in 0..arguments.len() {
                match (&arguments[i], &call.parameters[i].0) {
                    (Expr::Integer(_), ValueType::Integer) => (),
                    (Expr::Float(_), ValueType::Float) => (),
                    (Expr::String(_), ValueType::String) => (),
                    (Expr::Boolean(_), ValueType::Boolean) => (),
                    (Expr::Object(object_type, ..), ValueType::Class(class_type)) if object_type == class_type => (),
                    (expr, _) => {
                        let expr_type = self.expr_type(expr);

                        self.error.add(ParserErrType::CallArgumentIncorrectType(
                            self.tokens[token_pos - 1].clone(),
                            i,
                            expr_type,
                            name.to_string(),
                        ));
                    }
                }
            }

            return Ok(match &call.return_data_type {
                ReturnType::Integer => {
                    Expr::Integer(IntegerExpr::Call(call.id, arguments))
                }
                ReturnType::Float => {
                    Expr::Float(FloatExpr::Call(call.id, arguments))
                }
                ReturnType::String => {
                    Expr::String(StringExpr::Call(call.id, arguments))
                }
                ReturnType::Boolean => {
                    Expr::Boolean(BooleanExpr::Call(call.id, arguments))
                }
                ReturnType::Class(_) => {
                    todo!()
                }
                ReturnType::None => return Ok(Expr::None(NoneExpr::Call(call.id, arguments))),
            })
        }

        self.error.add(ParserErrType::CallFunctionNotFound(
            self.tokens[token_pos - 1].clone(),
            name.to_string(),
        ));
        Err(ParserStatus::Unwind)
    }

    fn data_type(&mut self) -> Result<ValueType, Option<Token>> {
        match self.current_token_type() {
            Some(TokenType::IntegerType) => Ok(ValueType::Integer),
            Some(TokenType::FloatType) => Ok(ValueType::Float),
            Some(TokenType::StringType) => Ok(ValueType::String),
            Some(TokenType::BooleanType) => Ok(ValueType::Boolean),
            Some(TokenType::Identifier(type_name)) => Ok(ValueType::Class(type_name.to_string())),
            _ => Err(self.tokens.get(self.current).cloned()),
        }
    }

    fn return_type(&mut self) -> Result<ReturnType, Option<Token>> {
        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::IntegerType,
                ..
            }) => Ok(ReturnType::Integer),
            Some(Token {
                token_type: TokenType::FloatType,
                ..
            }) => Ok(ReturnType::Float),
            Some(Token {
                token_type: TokenType::StringType,
                ..
            }) => Ok(ReturnType::String),
            Some(Token {
                token_type: TokenType::BooleanType,
                ..
            }) => Ok(ReturnType::Boolean),
            t => Err(t.cloned()),
        }
    }

    fn expr_type(&self, expr: &Expr) -> ReturnType {
        match expr {
            Expr::Integer(_) => ReturnType::Integer,
            Expr::Float(_) => ReturnType::Float,
            Expr::String(_) => ReturnType::String,
            Expr::Boolean(_) => ReturnType::Boolean,
            Expr::None(_) => ReturnType::None,
            Expr::Object(type_name, ..) => ReturnType::Class(type_name.to_string()),
        }
    }

    fn create_object(&mut self, class_name: String) -> Result<(Object, Vec<ConstructionType>), ParserStatus> {
        let declaration = match self.class_declarations.get(&class_name) {
            Some(declaration) => declaration,
            None => {
                panic!("Class not declared")
            }
        };

        let mut object = Object {
            class_declaration: class_name.clone(),
            properties: IndexMap::new(),
        };

        let mut types = vec![];

        for (name, value_type) in declaration.properties.clone() {
            match value_type {
                ValueType::Integer => {
                    object.properties.insert(name.to_string(), Value::Integer(self.integer_next_id));
                    self.integer_next_id += 1;
                    types.push(ConstructionType::Integer);
                }
                ValueType::Float => {
                    object.properties.insert(name.to_string(), Value::Float(self.float_next_id));
                    self.float_next_id += 1;
                    types.push(ConstructionType::Float);
                }
                ValueType::String => {
                    object.properties.insert(name.to_string(), Value::String(self.string_next_id));
                    self.string_next_id += 1;
                    types.push(ConstructionType::String);
                }
                ValueType::Boolean => {
                    object.properties.insert(name.to_string(), Value::Boolean(self.boolean_next_id));
                    self.boolean_next_id += 1;
                    types.push(ConstructionType::Boolean);
                }
                ValueType::Class(class_type) => {
                    let (this_object, these_types) = self.create_object(class_type)?;
                    types.push(ConstructionType::Class(these_types));
                    object.properties.insert(name, Value::Object(this_object));
                }
            }
        }

        Ok((object, types))
    }

    fn find_value(&self, name: &str) -> Option<Value> {
        for scope in self.value_stack.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone())
            }
        }
        None
    }

    fn get_variable_expr(&self, value: Value, name: String) -> Expr {
        match value {
            Value::Integer(id) => {
                Expr::Integer(IntegerExpr::Variable(id))
            }
            Value::Float(id) => {
                Expr::Float(FloatExpr::Variable(id))
            }
            Value::String(id) => {
                Expr::String(StringExpr::Variable(id))
            }
            Value::Boolean(id) => {
                Expr::Boolean(BooleanExpr::Variable(id))
            }
            Value::Object(obj) => {
                Expr::Object(
                    obj.class_declaration.clone(),
                    name.to_string(),
                    {
                        let mut expressions = vec![];
                        for (name, value) in obj.properties {
                            expressions.push(self.get_variable_expr(value, name));
                        }
                        expressions
                    }
                )
            }
        }
    }

    fn add_function_parameter(&mut self, value_type: &ValueType, name: &str, scope: &mut IndexMap<String, Value>) {
        match value_type {
            ValueType::Integer => {
                scope.insert(
                    name.to_string(),
                    Value::Integer(self.integer_next_id),
                );
                self.integer_next_id += 1;
            }
            ValueType::Float => {
                scope
                    .insert(name.to_string(), Value::Float(self.float_next_id));
                self.float_next_id += 1;
            }
            ValueType::String => {
                scope.insert(
                    name.to_string(),
                    Value::String(self.string_next_id),
                );
                self.string_next_id += 1;
            }
            ValueType::Boolean => {
                scope.insert(
                    name.to_string(),
                    Value::Boolean(self.boolean_next_id),
                );
                self.boolean_next_id += 1;
            }
            ValueType::Class(class_type) => {
                match self.class_declarations.remove(class_type) {
                    Some(cd) => {
                        let mut properties = IndexMap::new();

                        for (name, value_type) in &cd.properties {
                            self.add_function_parameter(value_type, name, &mut properties);
                        }

                        self.class_declarations.insert(class_type.to_string(), cd);

                        scope.insert(
                            name.to_string(),
                            Value::Object(Object {
                                class_declaration: class_type.to_string(),
                                properties,
                            }),
                        );
                    }
                    None => panic!("Function parameter of undefined class type")
                }
            }
        }
    }
}
