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
    function::Function,
    function_declaration::FunctionDeclaration,
    native_function::{
        cli_api::{CliFunctionNone, CliFunctionString},
        NativeFunctionNone, NativeFunctionString,
    },
    operator::{NumericOperator, StringOperator},
    parser_debug,
    return_type::ReturnType,
    start::Start,
    stmt::Stmt,
    token::{Token, TokenType},
    value_type::ValueType,
};
use rustc_hash::FxHashMap;

pub struct Parser {
    tokens: Vec<Token>,
    value_stack: Vec<FxHashMap<String, (ValueType, usize)>>,
    integer_next_id: usize,
    float_next_id: usize,
    string_next_id: usize,
    boolean_next_id: usize,
    function_declarations: FxHashMap<String, FunctionDeclaration>,
    current_function_declaration: Option<FunctionDeclaration>,
    functions: Vec<Function>,
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
            current_function_declaration: None,
            functions: vec![],
            error: ParserErr::new(),
            start: None,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn current_token(&self) -> Option<&Token> {
        let token = if let Some(t) = self.tokens.get(self.current) {
            Some(t)
        } else {
            None
        };
        token
    }

    fn current_token_type(&self) -> Option<&TokenType> {
        if let Some(t) = self.tokens.get(self.current) {
            Some(&t.token_type)
        } else {
            None
        }
    }

    fn consume_token_type(&mut self) -> Option<TokenType> {
        self.current += 1;
        let token = if let Some(t) = self.tokens.get(self.current - 1) {
            Some(t.token_type.clone())
        } else {
            None
        };
        token
    }

    fn previous_token(&self) -> Option<&Token> {
        let token = if let Some(t) = self.tokens.get(self.current - 1) {
            Some(t)
        } else {
            None
        };
        token
    }

    pub fn run(mut self) -> Result<(Stmt, Vec<Function>), ParserErr> {
        parser_debug!("Production rule path:");

        self.program();

        parser_debug!("Printing statements");

        #[cfg(debug_assertions)]
        println!("Start block: {:?}", self.start);

        if let Some(s) = self.start {
            if !self.error.had_error() {
                if let Some(stmt) = s.stmt {
                    return Ok((stmt, self.functions));
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
                Err(ParserStatus::Unwind) => {
                    // Attempt to synchronise
                    loop {
                        if let Some(TokenType::Start | TokenType::Function) | None =
                            self.current_token_type()
                        {
                            break;
                        }

                        self.current += 1;
                    }
                }
                Err(ParserStatus::End) => break,
            };
        }
    }

    fn global_declaration(&mut self) -> Result<(), ParserStatus> {
        debug_information!("global_declaration");

        match self.current_token() {
            Some(Token {
                token_type: TokenType::Start,
                ..
            }) => self.start_declaration(),
            Some(Token {
                token_type: TokenType::Function,
                ..
            }) => self.function_declaration(),
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

        let start_token = self.current_token().unwrap().clone();
        self.current += 1;

        // Add start value scope
        self.value_stack.push(FxHashMap::default());

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
        let function_token = self.current_token().unwrap().clone();
        self.current += 1;

        let function_name = match self.current_token() {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
            t => {
                self.error
                    .add(ParserErrType::FunctionDeclarationExpectedName(
                        function_token,
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        match self.current_token() {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error
                    .add(ParserErrType::FunctionDeclarationExpectedLeftParen(
                        self.previous_token().unwrap().clone(),
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
                                self.previous_token().unwrap().clone(),
                                t,
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };
                self.current += 1;

                let parameter_name = match self.current_token() {
                    Some(Token {
                        token_type: TokenType::Identifier(name),
                        ..
                    }) => name.clone(),
                    t => {
                        self.error
                            .add(ParserErrType::FunctionDeclarationExpectedParameterName(
                                function_token,
                                t.cloned(),
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };
                self.current += 1;

                parameters.push((parameter_data_type, parameter_name));

                match self.current_token() {
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
                                function_token,
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
                            self.previous_token().unwrap().clone(),
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
        let mut function_scope = FxHashMap::default();
        for parameter in &parameters {
            match parameter.0 {
                ValueType::Integer => {
                    function_scope.insert(
                        parameter.1.clone(),
                        (ValueType::Integer, self.integer_next_id),
                    );
                    self.integer_next_id += 1;
                }
                ValueType::Float => {
                    function_scope
                        .insert(parameter.1.clone(), (ValueType::Float, self.float_next_id));
                    self.float_next_id += 1;
                }
                ValueType::String => {
                    function_scope.insert(
                        parameter.1.clone(),
                        (ValueType::String, self.string_next_id),
                    );
                    self.string_next_id += 1;
                }
                ValueType::Boolean => {
                    function_scope.insert(
                        parameter.1.clone(),
                        (ValueType::Boolean, self.boolean_next_id),
                    );
                    self.boolean_next_id += 1;
                }
            }
        }
        self.value_stack.push(function_scope);

        let function_declaration = FunctionDeclaration {
            id: self.functions.len(),
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

        // Finally add function
        self.functions.push(Function { start: block });

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

        match self.current_token() {
            Some(Token {
                token_type: TokenType::SemiColon,
                ..
            }) => {
                self.current += 1;
                return Ok(statement);
            }
            t => self.error.add(ParserErrType::UnterminatedStatement(
                self.previous_token().unwrap().clone(),
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

        let equality = match self.current_token_type() {
            Some(TokenType::SemiColon) => None,
            _ => Some(self.equality()?),
        };

        Ok(Stmt::Return(match (function_ret_type, equality) {
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

        let left_paren = match self.current_token() {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::IfExpectedLeftParen(
                    self.previous_token().unwrap().clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };
        self.current += 1;

        let expression = self.equality()?;

        let right_paren = match self.current_token() {
            Some(Token {
                token_type: TokenType::RightParen,
                end,
                ..
            }) => *end,
            t => {
                self.error.add(ParserErrType::IfExpectedRightParen(
                    self.previous_token().unwrap().clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
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

        let left_paren = match self.current_token() {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::WhileExpectedLeftParen(
                    self.previous_token().unwrap().clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };

        self.current += 1;

        let expression = self.equality()?;

        let right_paren = match self.current_token() {
            Some(Token {
                token_type: TokenType::RightParen,
                end,
                ..
            }) => *end,
            t => {
                self.error.add(ParserErrType::WhileExpectedRightParen(
                    self.previous_token().unwrap().clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
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

        match self.current_token() {
            Some(Token {
                token_type: TokenType::LeftParen,
                ..
            }) => {
                self.current += 1;
            }
            t => {
                self.error.add(ParserErrType::ForExpectedLeftParen(
                    self.previous_token().unwrap().clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };

        self.value_stack.push(FxHashMap::default());
        let integer_point = self.integer_next_id;
        let float_point = self.float_next_id;
        let string_point = self.string_next_id;
        let boolean_point = self.boolean_next_id;

        let initialiser_statement = self.variable_declaration()?;

        let test_statement_start = match self.current_token() {
            Some(Token {
                token_type: TokenType::Comma,
                end,
                ..
            }) => *end,
            t => {
                self.error.add(ParserErrType::ForExpectedComma1(
                    self.previous_token().unwrap().clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };
        self.current += 1;

        let test_statement = self.equality()?;

        let test_statement_end = match self.current_token() {
            Some(Token {
                token_type: TokenType::Comma,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::ForExpectedComma2(
                    self.previous_token().unwrap().clone(),
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

        let update_statement = self.expression_statement()?;

        match self.current_token() {
            Some(Token {
                token_type: TokenType::RightParen,
                ..
            }) => {
                self.current += 1;
            }
            t => {
                self.error.add(ParserErrType::ForExpectedRightParen(
                    self.previous_token().unwrap().clone(),
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

        let open_brace_pos = match self.current_token() {
            Some(Token {
                token_type: TokenType::LeftBrace,
                ..
            }) => self.current,
            t => {
                self.error.add(ParserErrType::BlockExpectedLeftBrace(
                    self.previous_token().unwrap().clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let mut statements = vec![];
        self.value_stack.push(FxHashMap::default());

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
                Some(_) => statements.push(self.local_declaration()?),
                None => {
                    self.error.add(ParserErrType::BlockExpectedRightBrace(
                        self.tokens[open_brace_pos].clone(),
                        self.previous_token().unwrap().clone(),
                    ));

                    return Err(ParserStatus::End);
                }
            }
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("expression_statement");

        let expr = self.equality()?;

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

                let value = self.equality()?;

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
                                TokenType::SlashEqual => NumericAssignmentOperator::SlashEqual,
                                _ => panic!("Unreachable"),
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
                                TokenType::SlashEqual => NumericAssignmentOperator::SlashEqual,
                                _ => panic!("Unreachable"),
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

        let name = match self.current_token() {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
            t => {
                self.error
                    .add(ParserErrType::VariableDeclarationExpectedName(
                        self.previous_token().unwrap().clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        if let Some(_) = self.value_stack.last().unwrap().get(&name) {
            self.error
                .add(ParserErrType::VariableDeclarationAlreadyDeclared(
                    self.previous_token().unwrap().clone(),
                    name,
                ));
            return Err(ParserStatus::Unwind);
        }

        let equal_pos = match self.current_token() {
            Some(Token {
                token_type: TokenType::Equal,
                ..
            }) => self.current,
            t => {
                self.error
                    .add(ParserErrType::VariableDeclarationExpectedEqual(
                        self.previous_token().unwrap().clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let expr = self.equality()?;

        match expr {
            Expr::Integer(val) => {
                let id = self.integer_next_id;
                self.integer_next_id += 1;
                self.value_stack
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), (ValueType::Integer, id));
                Ok(Stmt::IntegerVariableDeclaration(val))
            }
            Expr::Float(val) => {
                let id = self.float_next_id;
                self.float_next_id += 1;
                self.value_stack
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), (ValueType::Float, id));
                Ok(Stmt::FloatVariableDeclaration(val))
            }
            Expr::String(val) => {
                let id = self.string_next_id;
                self.string_next_id += 1;
                self.value_stack
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), (ValueType::String, id));
                Ok(Stmt::StringVariableDeclaration(val))
            }
            Expr::Boolean(val) => {
                let id = self.boolean_next_id;
                self.boolean_next_id += 1;
                self.value_stack
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), (ValueType::Boolean, id));
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
        }
    }

    fn terminated_variable_declaration(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("terminated_variable_declaration");

        let variable_declaration = self.variable_declaration()?;

        match self.current_token() {
            Some(Token {
                token_type: TokenType::SemiColon,
                ..
            }) => {
                self.current += 1;
                return Ok(variable_declaration);
            }
            t => self.error.add(ParserErrType::UnterminatedStatement(
                self.previous_token().unwrap().clone(),
                t.cloned(),
            )),
        }

        Err(ParserStatus::End)
    }

    fn equality(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("equality");

        let mut left = self.comparision()?;

        loop {
            if let Some(TokenType::EqualEqual | TokenType::BangEqual) = self.current_token_type() {
                let comparator = self.current;
                self.current += 1;

                let right = self.comparision()?;

                let comparator_type = &self.tokens[comparator].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::IntegerBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => NumericComparision::Equal,
                                TokenType::BangEqual => NumericComparision::Inequal,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::FloatBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => NumericComparision::Equal,
                                TokenType::BangEqual => NumericComparision::Inequal,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(left_inside), Expr::String(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::StringBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => StringComparision::Equal,
                                TokenType::BangEqual => StringComparision::Inequal,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Boolean(left_inside), Expr::Boolean(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::BooleanBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => BooleanComparision::Equal,
                                TokenType::BangEqual => BooleanComparision::Inequal,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                            self.tokens[comparator].clone(),
                            left,
                            right,
                        ));

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
                let comparator = self.current;
                self.current += 1;

                let right = self.addsub()?;

                let comparator_type = &self.tokens[comparator].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::IntegerBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::MoreEqual => NumericComparision::MoreEqual,
                                TokenType::LessEqual => NumericComparision::LessEqual,
                                TokenType::More => NumericComparision::More,
                                TokenType::Less => NumericComparision::Less,
                                _ => panic!("Unreachable"),
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
                                TokenType::Less => NumericComparision::Less,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(_), Expr::String(_)) => {
                        self.error.add(ParserErrType::ComparisionInvalidForType(
                            self.tokens[comparator].clone(),
                            ReturnType::String,
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                    (Expr::Boolean(_), Expr::Boolean(_)) => {
                        self.error.add(ParserErrType::ComparisionInvalidForType(
                            self.tokens[comparator].clone(),
                            ReturnType::Boolean,
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                    (Expr::None(_), Expr::None(_)) => {
                        self.error.add(ParserErrType::ComparisionInvalidForType(
                            self.tokens[comparator].clone(),
                            ReturnType::None,
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                            self.tokens[comparator].clone(),
                            left,
                            right,
                        ));

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
                let operator = self.current;
                self.current += 1;

                let right = self.multdiv()?;

                let operator_type = &self.tokens[operator].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Integer(IntegerExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Plus => NumericOperator::Add,
                                TokenType::Minus => NumericOperator::Subtract,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Float(FloatExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Plus => NumericOperator::Add,
                                TokenType::Minus => NumericOperator::Subtract,
                                _ => panic!("Unreachable"),
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
                                        self.tokens[operator].clone(),
                                        ReturnType::String,
                                    ));

                                    return Err(ParserStatus::Unwind);
                                }
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Boolean(_), Expr::Boolean(_)) => {
                        self.error.add(ParserErrType::OperatorInvalidForType(
                            self.tokens[operator].clone(),
                            ReturnType::Boolean,
                        ));

                        return Err(ParserStatus::Unwind);
                    }
                    (Expr::None(_), Expr::None(_)) => {
                        self.error.add(ParserErrType::OperatorInvalidForType(
                            self.tokens[operator].clone(),
                            ReturnType::None,
                        ));

                        return Err(ParserStatus::Unwind);
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        self.error.add(ParserErrType::OperatorUnmatchingTypes(
                            self.tokens[operator].clone(),
                            left,
                            right,
                        ));

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

        let mut left = self.literal()?;

        loop {
            if let Some(TokenType::Star | TokenType::Slash) = self.current_token_type() {
                let operator = self.current;
                self.current += 1;

                let right = self.literal()?;

                let operator_type = &self.tokens[operator].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Integer(IntegerExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Star => NumericOperator::Multiply,
                                TokenType::Slash => NumericOperator::Divide,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Float(FloatExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Star => NumericOperator::Multiply,
                                TokenType::Slash => NumericOperator::Divide,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(_), Expr::String(_)) => {
                        self.error.add(ParserErrType::OperatorInvalidForType(
                            self.tokens[operator].clone(),
                            ReturnType::Boolean,
                        ));

                        return Err(ParserStatus::Unwind);
                    }
                    (Expr::Boolean(_), Expr::Boolean(_)) => {
                        self.error.add(ParserErrType::OperatorInvalidForType(
                            self.tokens[operator].clone(),
                            ReturnType::Boolean,
                        ));

                        return Err(ParserStatus::Unwind);
                    }
                    (Expr::None(_), Expr::None(_)) => {
                        self.error.add(ParserErrType::OperatorInvalidForType(
                            self.tokens[operator].clone(),
                            ReturnType::None,
                        ));

                        return Err(ParserStatus::Unwind);
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        self.error.add(ParserErrType::OperatorUnmatchingTypes(
                            self.tokens[operator].clone(),
                            left,
                            right,
                        ));

                        return Err(ParserStatus::Unwind);
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn literal(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("literal");

        match self.consume_token_type() {
            Some(TokenType::Integer(val)) => Ok(Expr::Integer(IntegerExpr::Literal(val))),
            Some(TokenType::Float(val)) => Ok(Expr::Float(FloatExpr::Literal(val))),
            Some(TokenType::String(val)) => Ok(Expr::String(StringExpr::Literal(val))),
            Some(TokenType::Boolean(val)) => Ok(Expr::Boolean(BooleanExpr::Literal(val))),
            Some(TokenType::Identifier(val)) => {
                if let Some(TokenType::LeftParen) = self.current_token_type() {
                    // Calling a function declared in this script
                    self.call(val.clone(), None, self.current)
                } else if let Some(TokenType::Colon) = self.current_token_type() {
                    // Calling a function within a module
                    self.current += 1;

                    match self.tokens.get(self.current) {
                        Some(Token {
                            token_type: TokenType::Identifier(name),
                            ..
                        }) => {
                            self.current += 1;

                            // Make sure the left paren is present
                            match self.tokens.get(self.current) {
                                Some(Token {
                                    token_type: TokenType::LeftParen,
                                    ..
                                }) => self.call(name.clone(), Some(val.clone()), self.current - 1),
                                t => {
                                    self.error.add(ParserErrType::ModuleExpectedLeftParen(
                                        self.previous_token().unwrap().clone(),
                                        t.cloned(),
                                    ));
                                    Err(ParserStatus::Unwind)
                                }
                            }
                        }
                        t => {
                            self.error.add(ParserErrType::ModuleExpectedIdentifier(
                                self.previous_token().unwrap().clone(),
                                t.cloned(),
                            ));
                            Err(ParserStatus::Unwind)
                        }
                    }
                } else {
                    for scope in self.value_stack.iter().rev() {
                        if let Some((value_type, id)) = scope.get(&val) {
                            match value_type {
                                ValueType::Integer => {
                                    return Ok(Expr::Integer(IntegerExpr::Variable(*id)))
                                }
                                ValueType::Float => {
                                    return Ok(Expr::Float(FloatExpr::Variable(*id)))
                                }
                                ValueType::String => {
                                    return Ok(Expr::String(StringExpr::Variable(*id)))
                                }
                                ValueType::Boolean => {
                                    return Ok(Expr::Boolean(BooleanExpr::Variable(*id)))
                                }
                            }
                        }
                    }

                    self.error.add(ParserErrType::VariableNotFound(
                        self.tokens[self.current - 1].clone(),
                        val,
                    ));
                    Err(ParserStatus::Unwind)
                }
            }
            _ => {
                self.error.add(ParserErrType::ExpectedLiteralVariableCall(
                    self.tokens[self.current - 2].clone(),
                    self.tokens.get(self.current - 1).cloned(),
                ));
                Err(ParserStatus::Unwind)
            }
        }
    }

    fn call(
        &mut self,
        name: String,
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
                let argument = self.equality()?;

                arguments.push(argument);

                match self.current_token() {
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
            match module.as_str() {
                "cli" => match name.as_str() {
                    "println" => {
                        if arguments.len() != 1 {
                            self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                                self.tokens[token_pos].clone(),
                                arguments.len(),
                                1,
                                "println".to_string(),
                            ));
                            return Err(ParserStatus::Unwind);
                        }
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Cli(
                            match arguments.pop() {
                                Some(Expr::Integer(arg)) => {
                                    CliFunctionNone::PrintLineInteger(Box::new(arg))
                                }
                                Some(Expr::Float(arg)) => {
                                    CliFunctionNone::PrintLineFloat(Box::new(arg))
                                }
                                Some(Expr::String(arg)) => {
                                    CliFunctionNone::PrintLineString(Box::new(arg))
                                }
                                Some(Expr::Boolean(arg)) => {
                                    CliFunctionNone::PrintLineBoolean(Box::new(arg))
                                }
                                None => CliFunctionNone::PrintLine,
                                _ => {
                                    self.error.add(ParserErrType::CallArgumentIncorrectType(
                                        self.tokens[token_pos].clone(),
                                        1,
                                        ReturnType::None,
                                        "println".to_string(),
                                    ));
                                    return Err(ParserStatus::Unwind);
                                }
                            },
                        ))));
                    }
                    "print" => {
                        if arguments.len() != 1 {
                            self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                                self.tokens[token_pos].clone(),
                                arguments.len(),
                                1,
                                "print".to_string(),
                            ));
                            return Err(ParserStatus::Unwind);
                        }
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Cli(
                            match arguments.pop() {
                                Some(Expr::Integer(arg)) => {
                                    CliFunctionNone::PrintInteger(Box::new(arg))
                                }
                                Some(Expr::Float(arg)) => {
                                    CliFunctionNone::PrintFloat(Box::new(arg))
                                }
                                Some(Expr::String(arg)) => {
                                    CliFunctionNone::PrintString(Box::new(arg))
                                }
                                Some(Expr::Boolean(arg)) => {
                                    CliFunctionNone::PrintBoolean(Box::new(arg))
                                }
                                _ => {
                                    self.error.add(ParserErrType::CallArgumentIncorrectType(
                                        self.tokens[token_pos].clone(),
                                        1,
                                        ReturnType::None,
                                        "println".to_string(),
                                    ));
                                    return Err(ParserStatus::Unwind);
                                }
                            },
                        ))));
                    }
                    "prompt" => {
                        if arguments.len() != 1 {
                            self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                                self.tokens[token_pos].clone(),
                                arguments.len(),
                                1,
                                "prompt".to_string(),
                            ));
                            return Err(ParserStatus::Unwind);
                        }
                        match arguments.pop() {
                            Some(Expr::String(argument)) => {
                                return Ok(Expr::String(StringExpr::NativeCall(
                                    NativeFunctionString::Cli(CliFunctionString::Prompt(Box::new(
                                        argument,
                                    ))),
                                )));
                            }
                            _ => {
                                self.error.add(ParserErrType::CallArgumentIncorrectType(
                                    self.tokens[token_pos].clone(),
                                    1,
                                    ReturnType::None,
                                    "prompt".to_string(),
                                ));
                                return Err(ParserStatus::Unwind);
                            }
                        }
                    }
                    name => {
                        self.error.add(ParserErrType::CallModuleFunctionNotFound(
                            self.tokens[token_pos].clone(),
                            name.to_string(),
                            module,
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                },
                module => {
                    self.error.add(ParserErrType::CallModuleNotFound(
                        self.tokens[token_pos - 2].clone(),
                        module.to_string(),
                    ));
                    return Err(ParserStatus::Unwind);
                }
            }
        }

        //Must be a zonkey function
        if let Some(function) = self.function_declarations.get(&name) {
            if arguments.len() != function.parameters.len() {
                self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                    self.tokens[token_pos - 1].clone(),
                    arguments.len(),
                    function.parameters.len(),
                    name,
                ));
                return Err(ParserStatus::Unwind);
            }

            // Check arguments evaluate to the same type as parameters
            for i in 0..arguments.len() {
                match (&arguments[i], &function.parameters[i].0) {
                    (Expr::Integer(_), ValueType::Integer) => (),
                    (Expr::Float(_), ValueType::Float) => (),
                    (Expr::String(_), ValueType::String) => (),
                    (Expr::Boolean(_), ValueType::Boolean) => (),
                    (expr, _) => {
                        let expr_type = self.expr_type(expr);

                        self.error.add(ParserErrType::CallArgumentIncorrectType(
                            self.tokens[token_pos - 1].clone(),
                            i,
                            expr_type,
                            name.clone(),
                        ));
                    }
                }
            }

            match function.return_data_type {
                ReturnType::Integer => {
                    return Ok(Expr::Integer(IntegerExpr::Call(function.id, arguments)))
                }
                ReturnType::Float => {
                    return Ok(Expr::Float(FloatExpr::Call(function.id, arguments)))
                }
                ReturnType::String => {
                    return Ok(Expr::String(StringExpr::Call(function.id, arguments)))
                }
                ReturnType::Boolean => {
                    return Ok(Expr::Boolean(BooleanExpr::Call(function.id, arguments)))
                }
                ReturnType::None => return Ok(Expr::None(NoneExpr::Call(function.id, arguments))),
            }
        }

        self.error.add(ParserErrType::CallFunctionNotFound(
            self.tokens[token_pos - 1].clone(),
            name.clone(),
        ));
        Err(ParserStatus::Unwind)
    }

    fn data_type(&mut self) -> Result<ValueType, Option<Token>> {
        match self.current_token() {
            Some(Token {
                token_type: TokenType::IntegerType,
                ..
            }) => Ok(ValueType::Integer),
            Some(Token {
                token_type: TokenType::FloatType,
                ..
            }) => Ok(ValueType::Float),
            Some(Token {
                token_type: TokenType::StringType,
                ..
            }) => Ok(ValueType::String),
            Some(Token {
                token_type: TokenType::BooleanType,
                ..
            }) => Ok(ValueType::Boolean),
            t => Err(t.cloned()),
        }
    }

    fn return_type(&mut self) -> Result<ReturnType, Option<Token>> {
        match self.current_token() {
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
        }
    }
}
