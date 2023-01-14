use self::err::ParserErr;
use crate::{
    debug_information, expr::Expr, literal::Literal, parser_debug, stmt::Stmt, token::Token,
    tree_walker::value::ValueType,
};
use std::collections::VecDeque;

pub mod err;

pub struct Parser {
    tokens: VecDeque<Token>,
    pub statements: VecDeque<Stmt>,
    #[cfg(debug_assertions)]
    debug: bool,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>, _debug: bool) -> Self {
        Self {
            tokens,
            statements: VecDeque::new(),
            #[cfg(debug_assertions)]
            debug: _debug,
        }
    }

    pub fn run(mut self) -> Result<Self, ParserErr> {
        parser_debug!(self.debug, "Production rule path:");

        self.program()?;

        parser_debug!(self.debug, "Printing statements");

        #[cfg(debug_assertions)]
        if self.debug {
            for (i, statement) in self.statements.iter().enumerate() {
                println!("  {}: {:?}", i + 1, statement);
            }
        }

        Ok(self)
    }

    fn program(&mut self) -> Result<(), ParserErr> {
        debug_information!(self.debug, "program");

        while self.tokens.front() != None {
            let declaration = self.declaration()?;
            self.statements.push_back(declaration);
        }

        Ok(())
    }

    fn declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "declaration");

        match self.tokens.front() {
            Some(
                Token::IntegerType | Token::StringType | Token::BooleanType | Token::FloatType,
            ) => self.terminated_variable_declaration(),
            Some(Token::Function) => {
                self.tokens.pop_front();
                self.function_declaration()
            }
            Some(Token::Start) => {
                self.tokens.pop_front();
                self.start_declaration()
            }
            _ => self.statement(),
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "statement");

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
        debug_information!(self.debug, "terminated_statement");

        let expression = match self.tokens.front() {
            Some(Token::Exit) => {
                self.tokens.pop_front();
                self.exit_statement()?
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

    fn start_declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "start_declaration");

        let block = Box::new(self.block()?);

        Ok(Stmt::Start(block))
    }

    fn function_declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "function_declaration");

        let function_name = if let Some(Token::Identifier(identifier)) = self.tokens.pop_front() {
            identifier
        } else {
            return Err(ParserErr::FunctionDeclarationMissingName);
        };

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::FunctionDeclarationMissingLeftParen),
        }

        let mut parameters = vec![];

        match self.tokens.front() {
            Some(Token::RightParen) => {
                self.tokens.pop_front();
            }
            _ => loop {
                let parameter_data_type = match self.data_type() {
                    Ok(data_type) => data_type,
                    Err(_) => return Err(ParserErr::FunctionDeclarationParameterBadDataType),
                };

                let parameter_name =
                    if let Some(Token::Identifier(identifier)) = self.tokens.pop_front() {
                        identifier
                    } else {
                        return Err(ParserErr::FunctionDeclarationParameterMissingName);
                    };

                parameters.push((parameter_data_type, parameter_name));

                match self.tokens.pop_front() {
                    Some(Token::Comma) => continue,
                    Some(Token::RightParen) => break,
                    _ => return Err(ParserErr::FunctionDeclarationExpectedCommaOrRightParen),
                }
            },
        }

        let block = Box::new(self.block()?);

        Ok(Stmt::FunctionDeclaration(function_name, parameters, block))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "if_statement");

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::IfMissingLeftParen),
        }

        let expression = self.equality()?;

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
        debug_information!(self.debug, "while_statement");

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::WhileMissingLeftParen),
        }

        let expression = self.equality()?;

        match self.tokens.pop_front() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::WhileMissingRightParen),
        }

        let block = Box::new(self.block()?);

        Ok(Stmt::While(expression, block))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "for_statement");

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::ForMissingLeftParen),
        }

        let initialiser_statement = self.variable_declaration()?;

        match self.tokens.pop_front() {
            Some(Token::Comma) => (),
            _ => return Err(ParserErr::ForMissingCommaAfterInitialiserStatement),
        }

        let test_statement = self.equality()?;

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

        if let Stmt::Block(b) = &mut block {
            b.push(update_statement);
        }

        Ok(Stmt::Block(vec![
            initialiser_statement,
            Stmt::While(test_statement, Box::new(block)),
        ]))
    }

    fn loop_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "loop_statement");

        let block = Box::new(self.block()?);

        Ok(Stmt::Loop(block))
    }

    fn block(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "block");

        match self.tokens.pop_front() {
            Some(Token::LeftBrace) => (),
            _ => return Err(ParserErr::ExpectedLeftBraceBeforeBlock),
        }

        let mut statements = vec![];

        loop {
            match self.tokens.front() {
                Some(Token::RightBrace) => {
                    self.tokens.pop_front();
                    return Ok(Stmt::Block(statements));
                }
                Some(_) => statements.push(self.declaration()?),
                None => return Err(ParserErr::ExpectedRightBraceAfterBlock),
            }
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "expression_statement");

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

                if let Expr::Variable(name) = expr {
                    Ok(Stmt::VariableAssignment(
                        name,
                        value,
                        assignment_operator.unwrap(),
                    ))
                } else {
                    Err(ParserErr::LeftValueNotVariable)
                }
            }
            _ => Ok(Stmt::Expression(expr)),
        }
    }

    fn exit_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "exit_statement");

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::ExitMissingLeftParen),
        }
        match self.tokens.pop_front() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::ExitMissingRightParen),
        }

        Ok(Stmt::Exit)
    }

    fn variable_declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "variable_declaration");

        let variable_data_type = match self.data_type() {
            Ok(data_type) => data_type,
            Err(_) => return Err(ParserErr::VariableDeclarationBadDataType),
        };

        let name = match self.tokens.pop_front() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(ParserErr::ExpectedVariableName),
        };

        match self.tokens.pop_front() {
            Some(Token::Equal) => (),
            _ => return Err(ParserErr::ExpectedVariableEqual),
        }

        let expr = self.equality()?;

        Ok(Stmt::VariableDeclaration(variable_data_type, name, expr))
    }

    fn terminated_variable_declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "terminated_variable_declaration");

        let variable_declaration = self.variable_declaration()?;

        if let Some(Token::SemiColon) = self.tokens.pop_front() {
            Ok(variable_declaration)
        } else {
            Err(ParserErr::UnterminatedStatement)
        }
    }

    fn equality(&mut self) -> Result<Expr, ParserErr> {
        debug_information!(self.debug, "equality");

        let mut left = self.comparision()?;

        loop {
            if let Some(Token::EqualEqual | Token::BangEqual) = self.tokens.front() {
                let operator = self.tokens.pop_front();

                let right = self.comparision()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator: operator.unwrap(),
                    right: Box::new(right),
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn comparision(&mut self) -> Result<Expr, ParserErr> {
        debug_information!(self.debug, "comparison");

        let mut left = self.addsub()?;

        loop {
            if let Some(Token::MoreEqual | Token::LessEqual | Token::Less | Token::More) =
                self.tokens.front()
            {
                let operator = self.tokens.pop_front();

                let right = self.addsub()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator: operator.unwrap(),
                    right: Box::new(right),
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn addsub(&mut self) -> Result<Expr, ParserErr> {
        debug_information!(self.debug, "addsub");

        let mut left = self.multdiv()?;

        loop {
            if let Some(Token::Minus | Token::Plus) = self.tokens.front() {
                let operator = self.tokens.pop_front();

                let right = self.multdiv()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator: operator.unwrap(),
                    right: Box::new(right),
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn multdiv(&mut self) -> Result<Expr, ParserErr> {
        debug_information!(self.debug, "multdiv");

        let mut left = self.literal()?;

        loop {
            if let Some(Token::Star | Token::Slash) = self.tokens.front() {
                let operator = self.tokens.pop_front();

                let right = self.literal()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator: operator.unwrap(),
                    right: Box::new(right),
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn literal(&mut self) -> Result<Expr, ParserErr> {
        debug_information!(self.debug, "literal");

        match self.tokens.pop_front() {
            Some(Token::Integer(val)) => Ok(Expr::Literal(Literal::Integer(val))),
            Some(Token::Float(val)) => Ok(Expr::Literal(Literal::Float(val))),
            Some(Token::String(val)) => Ok(Expr::Literal(Literal::String(val))),
            Some(Token::Boolean(val)) => Ok(Expr::Literal(Literal::Boolean(val))),
            Some(Token::Identifier(val)) => {
                if let Some(Token::LeftParen) = self.tokens.front() {
                    self.tokens.pop_front();
                    self.call(val)
                } else {
                    Ok(Expr::Variable(val))
                }
            }
            _ => return Err(ParserErr::ExpectedLiteral),
        }
    }

    fn call(&mut self, name: String) -> Result<Expr, ParserErr> {
        debug_information!(self.debug, "call");

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

        Ok(Expr::Call(name, arguments))
    }

    fn data_type(&mut self) -> Result<ValueType, ()> {
        match self.tokens.pop_front() {
            Some(Token::IntegerType) => Ok(ValueType::Integer),
            Some(Token::FloatType) => Ok(ValueType::Float),
            Some(Token::BooleanType) => Ok(ValueType::Boolean),
            Some(Token::StringType) => Ok(ValueType::String),
            _ => Err(()),
        }
    }
}
