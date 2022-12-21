use self::err::ParserErr;
use crate::{
    expr::Expr, literal::Literal, stmt::Stmt, token::Token, tree_walker::value::ValueType,
};
use std::{iter::Peekable, slice::Iter};

pub mod err;

pub struct Parser<'a> {
    tokens: &'a mut Peekable<Iter<'a, Token>>,
    pub statements: Vec<Stmt>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        Self {
            tokens,
            statements: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<Self, ParserErr> {
        self.statements = self.program()?;

        Ok(self)
    }

    fn program(&mut self) -> Result<Vec<Stmt>, ParserErr> {
        let mut statements = vec![];

        while self.tokens.peek() != None {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParserErr> {
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
        if let Some(Token::LeftBrace) = self.tokens.peek() {
            self.tokens.next();
            self.block()
        } else {
            self.terminated_statement()
        }
    }

    fn terminated_statement(&mut self) -> Result<Stmt, ParserErr> {
        let expression = match self.tokens.peek() {
            Some(Token::Print) => {
                self.tokens.next();
                self.print_statement()?
            }
            Some(Token::Exit) => {
                self.tokens.next();
                self.exit_statement()?
            }
            _ => self.expression_statement()?,
        };

        if let Some(Token::SemiColon) = self.tokens.next() {
            Ok(expression)
        } else {
            Err(ParserErr::UnterminatedStatement)
        }
    }

    fn block(&mut self) -> Result<Stmt, ParserErr> {
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
        let expr = self.equality()?;

        if let Some(Token::Equal) = self.tokens.peek() {
            self.tokens.next();

            let value = self.equality()?;

            if let Expr::Variable(name) = expr {
                return Ok(Stmt::VariableAssignment(name, value));
            } else {
                return Err(ParserErr::LeftValueNotVariable);
            }
        }

        Ok(Stmt::Expression(expr))
    }

    fn exit_statement(&mut self) -> Result<Stmt, ParserErr> {
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
        match self.tokens.next() {
            Some(Token::Integer(val)) => Ok(Expr::Literal(Literal::Integer(*val))),
            Some(Token::Float(val)) => Ok(Expr::Literal(Literal::Float(*val))),
            Some(Token::String(val)) => Ok(Expr::Literal(Literal::String(val.clone()))),
            Some(Token::Boolean(val)) => Ok(Expr::Literal(Literal::Boolean(*val))),
            Some(Token::Identifier(val)) => Ok(Expr::Variable(val.clone())),
            _ => Err(ParserErr::ExpectedLiteral),
        }
    }
}
