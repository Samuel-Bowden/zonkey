use crate::{abstract_syntax_tree::{AbstractSyntaxTree, Expr}, literal::Literal, token::token_type::TokenType};
use self::{err::TreeWalkerErr, value::Value};

pub mod err;
mod value;

pub struct TreeWalker<'a> {
    ast: &'a AbstractSyntaxTree,
}

impl<'a> TreeWalker<'a> {
    pub fn new(ast: &'a AbstractSyntaxTree) -> Self {
        Self {
            ast
        }
    }

    pub fn run(self) -> Result<Self, TreeWalkerErr> {
        println!("{}", Self::interpret(&self.ast.0)?);

        Ok(self)
    }

    fn interpret(expression: &Expr) -> Result<Value, TreeWalkerErr> {
        match expression {
            Expr::Binary { left, operator, right } => {
                match operator {
                    TokenType::Minus => {
                        Ok((Self::interpret(&left)? - Self::interpret(&right)?)?)
                    }
                    TokenType::Plus => {
                        Ok((Self::interpret(&left)? + Self::interpret(&right)?)?)
                    }
                    TokenType::Slash => {
                        Ok((Self::interpret(&left)? / Self::interpret(&right)?)?)
                    }
                    TokenType::Star => {
                        Ok((Self::interpret(&left)? * Self::interpret(&right)?)?)
                    }
                    TokenType::EqualEqual => {
                        Ok(Value::Boolean(Self::interpret(&left)?.equal(&Self::interpret(&right)?)?))
                    }
                    TokenType::BangEqual => {
                        Ok(Value::Boolean(!(Self::interpret(&left)?.equal(&Self::interpret(&right)?)?)))
                    }
                    TokenType::LessEqual => {
                        Ok(Value::Boolean(Self::interpret(&left)?.less_equal(&Self::interpret(&right)?)?))
                    }
                    TokenType::Less => {
                        Ok(Value::Boolean(Self::interpret(&left)?.less(&Self::interpret(&right)?)?))
                    }
                    TokenType::MoreEqual => {
                        Ok(Value::Boolean(Self::interpret(&left)?.more_equal(&Self::interpret(&right)?)?))
                    }
                    TokenType::More => {
                        Ok(Value::Boolean(Self::interpret(&left)?.more(&Self::interpret(&right)?)?))
                    }
                    _ => Err(TreeWalkerErr::UnsupportedOperator),
                }
            }
            Expr::Literal(Literal::Integer(val)) => {
                Ok(Value::Integer(*val))
            }
            Expr::Literal(Literal::Float(val)) => {
                Ok(Value::Float(*val))
            }
            Expr::Literal(Literal::String(val)) => {
                Ok(Value::String(val.clone()))
            }
            Expr::Literal(Literal::Boolean(val)) => {
                Ok(Value::Boolean(val.clone()))
            }
        }
    }
}
