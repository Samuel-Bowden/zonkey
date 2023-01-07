use self::err::ParserErr;
use crate::{
    debug_information, expr::Expr, literal::Literal, parser_debug, stmt::Stmt, token::Token,
    tree_walker::value::ValueType,
};
use std::{iter::Peekable, slice::Iter};

pub mod err;

pub struct Parser<'a> {
    tokens: &'a mut Peekable<Iter<'a, Token>>,
    pub statements: Vec<Stmt>,
    #[cfg(debug_assertions)]
    debug: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Peekable<Iter<'a, Token>>, _debug: bool) -> Self {
        Self {
            tokens,
            statements: Vec::new(),
            #[cfg(debug_assertions)]
            debug: _debug,
        }
    }

    pub fn run(mut self) -> Result<Self, ParserErr> {
        parser_debug!(self.debug, "Production rule path:");

        self.statements = self.program()?;

        parser_debug!(self.debug, "Printing statements");

        #[cfg(debug_assertions)]
        if self.debug {
            for (i, statement) in self.statements.iter().enumerate() {
                println!("  {}: {:?}", i + 1, statement);
            }
        }

        Ok(self)
    }

    fn program(&mut self) -> Result<Vec<Stmt>, ParserErr> {
        debug_information!(self.debug, "program");

        let mut statements = vec![];

        while self.tokens.peek() != None {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "declaration");

        if let Some(
            Token::IntegerType | Token::StringType | Token::BooleanType | Token::FloatType,
        ) = self.tokens.peek()
        {
            self.variable_declaration()
        } else {
            self.statement()
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "statement");

        match self.tokens.peek() {
            Some(Token::LeftBrace) => self.block(),
            Some(Token::If) => {
                self.tokens.next();
                self.if_statement()
            }
            Some(Token::While) => {
                self.tokens.next();
                self.while_statement()
            }
            Some(Token::Loop) => {
                self.tokens.next();
                self.loop_statement()
            }
            _ => Ok(self.terminated_statement()?),
        }
    }

    fn terminated_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "terminated_statement");

        let expression = match self.tokens.peek() {
            Some(Token::Print) => {
                self.tokens.next();
                self.print_statement()?
            }
            Some(Token::Exit) => {
                self.tokens.next();
                self.exit_statement()?
            }
            Some(Token::Break) => {
                self.tokens.next();
                Stmt::Break
            }
            Some(Token::Continue) => {
                self.tokens.next();
                Stmt::Continue
            }
            _ => self.expression_statement()?,
        };

        if let Some(Token::SemiColon) = self.tokens.next() {
            Ok(expression)
        } else {
            Err(ParserErr::UnterminatedStatement)
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "if_statement");

        match self.tokens.next() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::IfMissingLeftParen),
        }

        let expression = self.equality()?;

        match self.tokens.next() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::IfMissingRightParen),
        }

        let true_branch = Box::new(self.block()?);

        let false_branch = match self.tokens.peek() {
            Some(Token::Else) => {
                self.tokens.next();

                Some(Box::new(self.statement()?))
            }
            _ => None,
        };

        Ok(Stmt::If(expression, true_branch, false_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "while_statement");

        match self.tokens.next() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::WhileMissingLeftParen),
        }

        let expression = self.equality()?;

        match self.tokens.next() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::WhileMissingRightParen),
        }

        let block = Box::new(self.block()?);

        Ok(Stmt::While(expression, block))
    }

    fn loop_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "loop_statement");

        let block = Box::new(self.block()?);

        Ok(Stmt::Loop(block))
    }

    fn block(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "block");

        match self.tokens.next() {
            Some(Token::LeftBrace) => (),
            _ => return Err(ParserErr::ExpectedLeftBraceBeforeBlock),
        }

        let mut statements = vec![];

        loop {
            match self.tokens.peek() {
                Some(Token::RightBrace) => {
                    self.tokens.next();
                    return Ok(Stmt::Block(statements));
                }
                Some(_) => statements.push(self.declaration()?),
                None => return Err(ParserErr::ExpectedRightBraceAfterBlock),
            }
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "print_statement");

        match self.tokens.next() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::PrintMissingLeftParen),
        }

        let expression = self.equality()?;

        match self.tokens.next() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::PrintMissingRightParen),
        }

        Ok(Stmt::Print(expression))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "expression_statement");

        let expr = self.equality()?;

        match self.tokens.peek() {
            Some(
                Token::Equal
                | Token::PlusEqual
                | Token::MinusEqual
                | Token::StarEqual
                | Token::SlashEqual,
            ) => {
                let assignment_operator = self.tokens.next();

                let value = self.equality()?;

                if let Expr::Variable(name) = expr {
                    Ok(Stmt::VariableAssignment(
                        name,
                        value,
                        assignment_operator.unwrap().clone(),
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

        match self.tokens.next() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::ExitMissingLeftParen),
        }
        match self.tokens.next() {
            Some(Token::RightParen) => (),
            _ => return Err(ParserErr::ExitMissingRightParen),
        }

        Ok(Stmt::Exit)
    }

    fn variable_declaration(&mut self) -> Result<Stmt, ParserErr> {
        debug_information!(self.debug, "variable_declaration");

        let data_type = match self.tokens.next().unwrap() {
            Token::IntegerType => ValueType::Integer,
            Token::FloatType => ValueType::Float,
            Token::BooleanType => ValueType::Boolean,
            Token::StringType => ValueType::String,
            _ => panic!("Data type token should represent a data type."),
        };

        let name = match self.tokens.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(ParserErr::ExpectedVariableName),
        };

        match self.tokens.next() {
            Some(Token::Equal) => (),
            _ => return Err(ParserErr::ExpectedVariableEqual),
        }

        let expr = self.equality()?;

        if let Some(Token::SemiColon) = self.tokens.next() {
            Ok(Stmt::VariableDeclaration(data_type, name.clone(), expr))
        } else {
            Err(ParserErr::UnterminatedStatement)
        }
    }

    fn equality(&mut self) -> Result<Expr, ParserErr> {
        debug_information!(self.debug, "equality");

        let mut left = self.comparision()?;

        loop {
            if let Some(Token::EqualEqual | Token::BangEqual) = self.tokens.peek() {
                let operator = self.tokens.next();

                let right = self.comparision()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator: operator.unwrap().clone(),
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
                self.tokens.peek()
            {
                let operator = self.tokens.next();

                let right = self.addsub()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator: operator.unwrap().clone(),
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
            if let Some(Token::Minus | Token::Plus) = self.tokens.peek() {
                let operator = self.tokens.next();

                let right = self.multdiv()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator: operator.unwrap().clone(),
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
            if let Some(Token::Star | Token::Slash) = self.tokens.peek() {
                let operator = self.tokens.next();

                let right = self.literal()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator: operator.unwrap().clone(),
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

        match self.tokens.next() {
            Some(Token::Integer(val)) => Ok(Expr::Literal(Literal::Integer(*val))),
            Some(Token::Float(val)) => Ok(Expr::Literal(Literal::Float(*val))),
            Some(Token::String(val)) => Ok(Expr::Literal(Literal::String(val.clone()))),
            Some(Token::Boolean(val)) => Ok(Expr::Literal(Literal::Boolean(*val))),
            Some(Token::Identifier(val)) => Ok(Expr::Variable(val.clone())),
            _ => return Err(ParserErr::ExpectedLiteral),
        }
    }
}
