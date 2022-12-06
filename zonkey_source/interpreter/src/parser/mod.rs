use crate::{token::{Token, token_type::TokenType}, abstract_syntax_tree::{AbstractSyntaxTree, Expr}};
use self::err::ParserErr;

pub mod err;

pub struct Parser<'a> {
    tokens: &'a mut std::slice::Iter<'a, Token>,
    pub abstract_syntax_tree: Option<AbstractSyntaxTree>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut std::slice::Iter<'a, Token>) -> Self {
        Self {
            tokens,
            abstract_syntax_tree: None,
        }
    }

    pub fn run(mut self) -> Result<Self, ParserErr> {
        self.abstract_syntax_tree = Some(AbstractSyntaxTree(
            self.expression()?
        ));

        Ok(self)
    }

    fn expression(&mut self) -> Result<Expr, ParserErr> {
        self.arithmetic()
    }

    fn arithmetic(&mut self) -> Result<Expr, ParserErr> {
        let mut left = self.literal()?;

        loop {
            let operator = self.tokens.next();

            if let Some(Token { token_type: TokenType::Minus | TokenType::Plus, literal: _ }) = operator {
                let right = self.literal()?;

                left = Expr::Binary { 
                    left: Box::new(left),
                    operator: operator.unwrap().token_type.clone(),
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
            Some(Token { token_type: TokenType::Integer, literal: val }) => {
               Ok(Expr::Literal(val.clone().unwrap()))
            }
            Some(Token { token_type: TokenType::Float, literal: val }) => {
               Ok(Expr::Literal(val.clone().unwrap()))
            }
            _ => Err(ParserErr::ExpectedLiteral),
        }
    }
}
