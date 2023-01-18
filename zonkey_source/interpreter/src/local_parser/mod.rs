use self::err::ParserErr;
use crate::{
    assignment_operator::{
        BooleanAssignmentOperator, NumericAssignmentOperator, StringAssignmentOperator,
    },
    comparison::{BooleanComparision, NumericComparision, StringComparision},
    debug_information,
    expr::{BooleanExpr, Expr, FloatExpr, IntegerExpr, NoneExpr, StringExpr},
    function_declaration::FunctionDeclaration,
    native_function::{
        cli_api::{CliFunctionNone, CliFunctionString, CliFunctionInteger},
        NativeFunctionNone, NativeFunctionString, NativeFunctionInteger,
    },
    operator::{NumericOperator, StringOperator},
    parser_debug,
    stmt::Stmt,
    token::Token,
    value_type::ValueType,
};
use std::collections::{HashMap, VecDeque};

pub mod err;

pub struct LocalParser<'a> {
    tokens: VecDeque<Token>,
    pub statements: Vec<Stmt>,
    value_stack: Vec<HashMap<String, (ValueType, usize)>>,
    integer_next_id: usize,
    float_next_id: usize,
    string_next_id: usize,
    boolean_next_id: usize,
    function_declarations: &'a HashMap<String, FunctionDeclaration>,
    function_declaration: Option<&'a FunctionDeclaration>,
}

impl<'a> LocalParser<'a> {
    pub fn new(
        tokens: VecDeque<Token>,
        function_declarations: &'a HashMap<String, FunctionDeclaration>,
    ) -> Self {
        Self {
            tokens,
            statements: vec![],
            value_stack: vec![HashMap::new()],
            integer_next_id: 0,
            float_next_id: 0,
            string_next_id: 0,
            boolean_next_id: 0,
            function_declarations,
            function_declaration: None,
        }
    }

    pub fn new_function(
        tokens: VecDeque<Token>,
        function_declarations: &'a HashMap<String, FunctionDeclaration>,
        function_declaration: &'a FunctionDeclaration,
    ) -> Self {
        let mut value_stack = vec![HashMap::new()];

        let mut integer_next_id = 0;
        let mut float_next_id = 0;
        let mut string_next_id = 0;
        let mut boolean_next_id = 0;

        for parameter in &function_declaration.parameters {
            match parameter.0 {
                ValueType::Integer => {
                    value_stack
                        .last_mut()
                        .unwrap()
                        .insert(parameter.1.clone(), (ValueType::Integer, integer_next_id));
                    integer_next_id += 1;
                }
                ValueType::Float => {
                    value_stack
                        .last_mut()
                        .unwrap()
                        .insert(parameter.1.clone(), (ValueType::Float, float_next_id));
                    float_next_id += 1;
                }
                ValueType::String => {
                    value_stack
                        .last_mut()
                        .unwrap()
                        .insert(parameter.1.clone(), (ValueType::String, string_next_id));
                    string_next_id += 1;
                }
                ValueType::Boolean => {
                    value_stack
                        .last_mut()
                        .unwrap()
                        .insert(parameter.1.clone(), (ValueType::Boolean, boolean_next_id));
                    boolean_next_id += 1;
                }
            }
        }

        Self {
            tokens,
            statements: vec![],
            value_stack,
            integer_next_id,
            float_next_id,
            string_next_id,
            boolean_next_id,
            function_declarations,
            function_declaration: Some(function_declaration),
        }
    }

    pub fn run(mut self) -> Result<Vec<Stmt>, ParserErr> {
        parser_debug!("Production rule path:");

        self.program()?;

        parser_debug!("Printing statements");

        #[cfg(debug_assertions)]
        for (i, statement) in self.statements.iter().enumerate() {
            println!("  {}: {:?}", i + 1, statement);
        }

        Ok(self.statements)
    }

    fn program(&mut self) -> Result<(), ParserErr> {
        debug_information!("program");

        while self.tokens.front() != None {
            let declaration = self.declaration()?;
            self.statements.push(declaration);
        }

        Ok(())
    }

    fn declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("declaration");

        match self.tokens.front() {
            Some(Token::Let) => self.terminated_variable_declaration(),
            _ => self.statement(),
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("statement");

        match self.tokens.front() {
            Some(Token::LeftBrace) => self.block(),
            Some(Token::If) => {
                self.tokens.pop_front();
                self.if_statement()
            }
            Some(Token::While) => {
                self.tokens.pop_front();
                self.while_statement()
            }
            Some(Token::Loop) => {
                self.tokens.pop_front();
                self.loop_statement()
            }
            Some(Token::For) => {
                self.tokens.pop_front();
                self.for_statement()
            }
            _ => Ok(self.terminated_statement()?),
        }
    }

    fn terminated_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("terminated_statement");

        let expression = match self.tokens.front() {
            Some(Token::Return) => {
                self.tokens.pop_front();
                self.return_statement()?
            }
            Some(Token::Break) => {
                self.tokens.pop_front();
                Stmt::Break
            }
            Some(Token::Continue) => {
                self.tokens.pop_front();
                Stmt::Continue
            }
            _ => self.expression_statement()?,
        };

        if let Some(Token::SemiColon) = self.tokens.pop_front() {
            Ok(expression)
        } else {
            Err(ParserErr::UnterminatedStatement)
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("return_statement");

        match self.tokens.front() {
            Some(Token::SemiColon) => Ok(Stmt::Return(None)),
            _ => Ok(Stmt::Return(Some(
                    if let Some(function) = self.function_declaration {
                        match (&function.return_data_type, self.equality()?) {
                            (Some(ValueType::Integer), Expr::Integer(expr)) => Expr::Integer(expr),
                            (Some(ValueType::Float), Expr::Float(expr)) => Expr::Float(expr),
                            (Some(ValueType::String), Expr::String(expr)) => Expr::String(expr),
                            (Some(ValueType::Boolean), Expr::Boolean(expr)) => Expr::Boolean(expr),
                            (None, Expr::None(expr)) => Expr::None(expr),
                            _ => panic!("Function return expression does not match data type of declaration")
                        }
                    } else {
                        if let Expr::None(expr) = self.equality()? {
                            Expr::None(expr)
                        } else {
                            panic!("Function return expression does not match data type of declaration")
                        }
                    }
            ))),
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("if_statement");

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::IfMissingLeftParen),
        }

        let expression = self.equality()?;

        let expression = if let Expr::Boolean(expr) = expression {
            expr
        } else {
            panic!("If condition must evaluate to a boolean")
        };

        match self.tokens.pop_front() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::IfMissingRightParen),
        }

        let true_branch = Box::new(self.block()?);

        let false_branch = match self.tokens.front() {
            Some(Token::Else) => {
                self.tokens.pop_front();

                Some(Box::new(self.statement()?))
            }
            _ => None,
        };

        Ok(Stmt::If(expression, true_branch, false_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("while_statement");

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::WhileMissingLeftParen),
        }

        let expression = self.equality()?;

        let expression = if let Expr::Boolean(expr) = expression {
            expr
        } else {
            panic!("While condition must evaluate to a boolean")
        };

        match self.tokens.pop_front() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::WhileMissingRightParen),
        }

        let block = Box::new(self.block()?);

        Ok(Stmt::While(expression, block))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("for_statement");

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::ForMissingLeftParen),
        }

        self.value_stack.push(HashMap::new());
        let integer_point = self.integer_next_id;
        let float_point = self.float_next_id;
        let string_point = self.string_next_id;
        let boolean_point = self.boolean_next_id;

        let initialiser_statement = self.variable_declaration()?;

        match self.tokens.pop_front() {
            Some(Token::Comma) => (),
            _ => return Err(ParserErr::ForMissingCommaAfterInitialiserStatement),
        }

        let test_statement = if let Expr::Boolean(test_statement) = self.equality()? {
            test_statement
        } else {
            panic!("If condition must evaluate to a boolean")
        };

        match self.tokens.pop_front() {
            Some(Token::Comma) => (),
            _ => return Err(ParserErr::ForMissingCommaAfterTestStatement),
        }

        let update_statement = self.expression_statement()?;

        match self.tokens.pop_front() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::ForMissingRightParen),
        }

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

    fn loop_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("loop_statement");

        let block = Box::new(self.block()?);

        Ok(Stmt::Loop(block))
    }

    fn block(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("block");

        match self.tokens.pop_front() {
            Some(Token::LeftBrace) => (),
            _ => return Err(ParserErr::ExpectedLeftBraceBeforeBlock),
        }

        let mut statements = vec![];
        self.value_stack.push(HashMap::new());

        let integer_point = self.integer_next_id;
        let float_point = self.float_next_id;
        let string_point = self.string_next_id;
        let boolean_point = self.boolean_next_id;

        loop {
            match self.tokens.front() {
                Some(Token::RightBrace) => {
                    self.tokens.pop_front();
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
                Some(_) => statements.push(self.declaration()?),
                None => return Err(ParserErr::ExpectedRightBraceAfterBlock),
            }
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("expression_statement");

        let expr = self.equality()?;

        match self.tokens.front() {
            Some(
                Token::Equal
                | Token::PlusEqual
                | Token::MinusEqual
                | Token::StarEqual
                | Token::SlashEqual,
            ) => {
                let assignment_operator = self.tokens.pop_front();

                let value = self.equality()?;

                match (expr, value) {
                    (Expr::Integer(IntegerExpr::Variable(id)), Expr::Integer(val)) => {
                        Ok(Stmt::IntegerVariableAssignment(
                            id,
                            val,
                            match assignment_operator {
                                Some(Token::Equal) => NumericAssignmentOperator::Equal,
                                Some(Token::PlusEqual) => NumericAssignmentOperator::PlusEqual,
                                Some(Token::MinusEqual) => NumericAssignmentOperator::MinusEqual,
                                Some(Token::StarEqual) => NumericAssignmentOperator::StarEqual,
                                Some(Token::SlashEqual) => NumericAssignmentOperator::SlashEqual,
                                _ => panic!("Shouldn't happen"),
                            },
                        ))
                    }
                    (Expr::Float(FloatExpr::Variable(id)), Expr::Float(val)) => {
                        Ok(Stmt::FloatVariableAssignment(
                            id,
                            val,
                            match assignment_operator {
                                Some(Token::Equal) => NumericAssignmentOperator::Equal,
                                Some(Token::PlusEqual) => NumericAssignmentOperator::PlusEqual,
                                Some(Token::MinusEqual) => NumericAssignmentOperator::MinusEqual,
                                Some(Token::StarEqual) => NumericAssignmentOperator::StarEqual,
                                Some(Token::SlashEqual) => NumericAssignmentOperator::SlashEqual,
                                _ => panic!("Shouldn't happen"),
                            },
                        ))
                    }
                    (Expr::String(StringExpr::Variable(id)), Expr::String(val)) => {
                        Ok(Stmt::StringVariableAssignment(
                            id,
                            val,
                            match assignment_operator {
                                Some(Token::Equal) => StringAssignmentOperator::Equal,
                                Some(Token::PlusEqual) => StringAssignmentOperator::PlusEqual,
                                Some(Token::MinusEqual) => panic!("Cannot use -= with strings"),
                                Some(Token::StarEqual) => panic!("Cannot use *= with strings"),
                                Some(Token::SlashEqual) => panic!("Cannot use /= with strings"),
                                _ => panic!("Shouldn't happen"),
                            },
                        ))
                    }
                    (Expr::Boolean(BooleanExpr::Variable(id)), Expr::Boolean(val)) => {
                        Ok(Stmt::BooleanVariableAssignment(
                            id,
                            val,
                            match assignment_operator {
                                Some(Token::Equal) => BooleanAssignmentOperator::Equal,
                                Some(Token::PlusEqual) => panic!("Cannot use += with booleans"),
                                Some(Token::MinusEqual) => panic!("Cannot use -= with booleans"),
                                Some(Token::StarEqual) => panic!("Cannot use *= with booleans"),
                                Some(Token::SlashEqual) => panic!("Cannot use /= with booleans"),
                                _ => panic!("Shouldn't happen"),
                            },
                        ))
                    }
                    _ => panic!("Type error variable assignment"),
                }
            }
            _ => Ok(Stmt::Expression(expr)),
        }
    }

    fn variable_declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("variable_declaration");

        self.tokens.pop_front();

        let name = match self.tokens.pop_front() {
            Some(Token::Identifier(name)) => name,
            other => return Err(ParserErr::ExpectedVariableName(other)),
        };

        if let Some(_) = self.value_stack.last().unwrap().get(&name) {
            panic!("Variable already declared");
        }

        match self.tokens.pop_front() {
            Some(Token::Equal) => (),
            _ => return Err(ParserErr::ExpectedVariableEqual),
        }

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
                panic!("Cannot not assign the value None to variable");
            }
        }
    }

    fn terminated_variable_declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!("terminated_variable_declaration");

        let variable_declaration = self.variable_declaration()?;

        if let Some(Token::SemiColon) = self.tokens.pop_front() {
            Ok(variable_declaration)
        } else {
            Err(ParserErr::UnterminatedStatement)
        }
    }

    fn equality(&mut self) -> Result<Expr, ParserErr> {
        debug_information!("equality");

        let mut left = self.comparision()?;

        loop {
            if let Some(Token::EqualEqual | Token::BangEqual) = self.tokens.front() {
                let comparator = self.tokens.pop_front();
                let right = self.comparision()?;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::IntegerBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator {
                                Some(Token::EqualEqual) => NumericComparision::Equal,
                                Some(Token::BangEqual) => NumericComparision::Inequal,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::FloatBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator {
                                Some(Token::EqualEqual) => NumericComparision::Equal,
                                Some(Token::BangEqual) => NumericComparision::Inequal,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(left_inside), Expr::String(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::StringBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator {
                                Some(Token::EqualEqual) => StringComparision::Equal,
                                Some(Token::BangEqual) => StringComparision::Inequal,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Boolean(left_inside), Expr::Boolean(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::BooleanBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator {
                                Some(Token::EqualEqual) => BooleanComparision::Equal,
                                Some(Token::BangEqual) => BooleanComparision::Inequal,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    _ => panic!("Type error comparision"),
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn comparision(&mut self) -> Result<Expr, ParserErr> {
        debug_information!("comparison");

        let mut left = self.addsub()?;

        loop {
            if let Some(Token::MoreEqual | Token::LessEqual | Token::Less | Token::More) =
                self.tokens.front()
            {
                let comparator = self.tokens.pop_front();
                let right = self.addsub()?;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::IntegerBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator {
                                Some(Token::MoreEqual) => NumericComparision::MoreEqual,
                                Some(Token::LessEqual) => NumericComparision::LessEqual,
                                Some(Token::More) => NumericComparision::More,
                                Some(Token::Less) => NumericComparision::Less,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::FloatBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator {
                                Some(Token::MoreEqual) => NumericComparision::MoreEqual,
                                Some(Token::LessEqual) => NumericComparision::LessEqual,
                                Some(Token::More) => NumericComparision::More,
                                Some(Token::Less) => NumericComparision::Less,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(_), Expr::String(_)) => match comparator {
                        Some(Token::MoreEqual) => panic!("Cannot use <= with strings"),
                        Some(Token::LessEqual) => panic!("Cannot use >= with strings"),
                        Some(Token::More) => panic!("Cannot use < with strings"),
                        Some(Token::Less) => panic!("Cannot use > with strings"),
                        _ => panic!("Unreachable"),
                    },
                    (Expr::Boolean(_), Expr::Boolean(_)) => match comparator {
                        Some(Token::MoreEqual) => panic!("Cannot use <= with booleans"),
                        Some(Token::LessEqual) => panic!("Cannot use >= with booleans"),
                        Some(Token::More) => panic!("Cannot use < with booleans"),
                        Some(Token::Less) => panic!("Cannot use > with booleans"),
                        _ => panic!("Unreachable"),
                    },
                    _ => panic!("Type error comparision"),
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn addsub(&mut self) -> Result<Expr, ParserErr> {
        debug_information!("addsub");

        let mut left = self.multdiv()?;

        loop {
            if let Some(Token::Minus | Token::Plus) = self.tokens.front() {
                let operator = self.tokens.pop_front();

                let right = self.multdiv()?;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Integer(IntegerExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator {
                                Some(Token::Plus) => NumericOperator::Add,
                                Some(Token::Minus) => NumericOperator::Subtract,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Float(FloatExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator {
                                Some(Token::Plus) => NumericOperator::Add,
                                Some(Token::Minus) => NumericOperator::Subtract,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(left_inside), Expr::String(right_inside)) => {
                        left = Expr::String(StringExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator {
                                Some(Token::Plus) => StringOperator::Add,
                                Some(Token::Minus) => panic!("Cannot subtract strings"),
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Boolean(_), Expr::Boolean(_)) => {
                        panic!("Cannot do add or subtract booleans")
                    }
                    _ => panic!("Type error addsub"),
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn multdiv(&mut self) -> Result<Expr, ParserErr> {
        debug_information!("multdiv");

        let mut left = self.literal()?;

        loop {
            if let Some(Token::Star | Token::Slash) = self.tokens.front() {
                let operator = self.tokens.pop_front();

                let right = self.literal()?;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Integer(IntegerExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator {
                                Some(Token::Star) => NumericOperator::Multiply,
                                Some(Token::Slash) => NumericOperator::Divide,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Float(FloatExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator {
                                Some(Token::Star) => NumericOperator::Multiply,
                                Some(Token::Slash) => NumericOperator::Divide,
                                _ => panic!("Unreachable"),
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(_), Expr::String(_)) => {
                        panic!("Cannot multiply or divide strings")
                    }
                    (Expr::Boolean(_), Expr::Boolean(_)) => {
                        panic!("Cannot multiply or divide booleans")
                    }
                    _ => panic!("Type error multdiv"),
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn literal(&mut self) -> Result<Expr, ParserErr> {
        debug_information!("literal");

        match self.tokens.pop_front() {
            Some(Token::Integer(val)) => Ok(Expr::Integer(IntegerExpr::Literal(val))),
            Some(Token::Float(val)) => Ok(Expr::Float(FloatExpr::Literal(val))),
            Some(Token::String(val)) => Ok(Expr::String(StringExpr::Literal(val))),
            Some(Token::Boolean(val)) => Ok(Expr::Boolean(BooleanExpr::Literal(val))),
            Some(Token::Identifier(val)) => {
                if let Some(Token::LeftParen) = self.tokens.front() {
                    self.tokens.pop_front();
                    self.call(val, None)
                } else if let Some(Token::ColonColon) = self.tokens.front() {
                    self.tokens.pop_front();
                    if let Some(Token::Identifier(name)) = self.tokens.pop_front() {
                        self.tokens.pop_front();
                        self.call(name, Some(val))
                    } else {
                        panic!("Expected an identifier for function within module")
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

                    panic!("Variable not found {val}");
                }
            }
            val => return Err(ParserErr::ExpectedLiteral(val)),
        }
    }

    fn call(&mut self, name: String, module: Option<String>) -> Result<Expr, ParserErr> {
        debug_information!("call");

        let mut arguments = vec![];

        match self.tokens.front() {
            Some(Token::RightParen) => {
                self.tokens.pop_front();
            }
            _ => loop {
                let argument = self.equality()?;

                arguments.push(argument);

                match self.tokens.pop_front() {
                    Some(Token::Comma) => continue,
                    Some(Token::RightParen) => break,
                    _ => return Err(ParserErr::CallExpectedCommaOrRightParen),
                }
            },
        }

        if let Some(module) = module {
            match module.as_str() {
                "cli" => match name.as_str() {
                    "println" => {
                        if arguments.len() > 1 {
                            panic!(
                                "println expected 0 or 1 arguments but was given {}",
                                arguments.len()
                            );
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
                                _ => panic!("Invalid argument for println"),
                            },
                        ))));
                    }
                    "print" => {
                        if arguments.len() != 1 {
                            panic!("Incorrect amount of arguments for print");
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
                                _ => panic!("Invalid argument for print"),
                            },
                        ))));
                    }
                    "prompt" => {
                        if arguments.len() != 1 {
                            panic!("Incorrect amount of arguments for prompt");
                        }
                        if let Some(Expr::String(argument)) = arguments.pop() {
                            return Ok(Expr::String(StringExpr::NativeCall(
                                NativeFunctionString::Cli(CliFunctionString::Prompt(Box::new(
                                    argument,
                                ))),
                            )));
                        } else {
                            panic!("Incorrect argument to prompt");
                        }
                    }
                    "prompt_int" => {
                        if arguments.len() != 1 {
                            panic!("Incorrect amount of arguments for prompt");
                        }
                        if let Some(Expr::String(argument)) = arguments.pop() {
                            return Ok(Expr::Integer(IntegerExpr::NativeCall(
                                NativeFunctionInteger::Cli(CliFunctionInteger::Prompt(Box::new(
                                    argument,
                                ))),
                            )));
                        } else {
                            panic!("Incorrect argument to prompt");
                        }
                    }
                    _ => panic!("Function does not exist inside CLI API"),
                },
                module => panic!("Invalid module {module}"),
            }
        }

        //Must be a zonkey function
        if let Some(function) = self.function_declarations.get(&name) {
            if arguments.len() != function.parameters.len() {
                panic!(
                    "println expected {} arguments but was given {}",
                    function.parameters.len(),
                    arguments.len()
                );
            }

            // Check arguments evaluate to the same type as parameters
            for i in 0..arguments.len() {
                match (&arguments[i], &function.parameters[i].0) {
                    (Expr::Integer(_), ValueType::Integer) => (),
                    (Expr::Float(_), ValueType::Float) => (),
                    (Expr::String(_), ValueType::String) => (),
                    (Expr::Boolean(_), ValueType::Boolean) => (),
                    _ => panic!(
                        "Argument {} recieves an expression of the wrong type in call",
                        function.parameters[i].1
                    ),
                }
            }

            match function.return_data_type {
                Some(ValueType::Integer) => {
                    return Ok(Expr::Integer(IntegerExpr::Call(function.id, arguments)))
                }
                Some(ValueType::Float) => {
                    return Ok(Expr::Float(FloatExpr::Call(function.id, arguments)))
                }
                Some(ValueType::String) => {
                    return Ok(Expr::String(StringExpr::Call(function.id, arguments)))
                }
                Some(ValueType::Boolean) => {
                    return Ok(Expr::Boolean(BooleanExpr::Call(function.id, arguments)))
                }
                None => return Ok(Expr::None(NoneExpr::Call(function.id, arguments))),
            }
        }

        panic!("Zonkey function has not been declared");
    }
}
