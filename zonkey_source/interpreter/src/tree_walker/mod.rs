use self::{err::TreeWalkerErr, value::Value};
use crate::{expr::Expr, literal::Literal, token::Token};
use std::slice::Iter;

pub mod err;
mod value;

pub struct TreeWalker<'a> {
    expressions: &'a mut Iter<'a, Expr>,
}

impl<'a> TreeWalker<'a> {
    pub fn new(expressions: &'a mut Iter<'a, Expr>) -> Self {
        Self { expressions }
    }

    pub fn run(self) -> Result<(), TreeWalkerErr> {
        for expression in self.expressions {
            println!("{}", Self::interpret(&expression)?);
        }

        Ok(())
    }

    fn interpret(expression: &Expr) -> Result<Value, TreeWalkerErr> {
        match expression {
            Expr::Binary {
                left,
                operator,
                right,
            } => match operator {
                Token::Minus => Ok((Self::interpret(&left)? - Self::interpret(&right)?)?),
                Token::Plus => Ok((Self::interpret(&left)? + Self::interpret(&right)?)?),
                Token::Slash => Ok((Self::interpret(&left)? / Self::interpret(&right)?)?),
                Token::Star => Ok((Self::interpret(&left)? * Self::interpret(&right)?)?),
                Token::EqualEqual => Ok(Value::Boolean(
                    Self::interpret(&left)?.equal(&Self::interpret(&right)?)?,
                )),
                Token::BangEqual => Ok(Value::Boolean(
                    !(Self::interpret(&left)?.equal(&Self::interpret(&right)?)?),
                )),
                Token::LessEqual => Ok(Value::Boolean(
                    Self::interpret(&left)?.less_equal(&Self::interpret(&right)?)?,
                )),
                Token::Less => Ok(Value::Boolean(
                    Self::interpret(&left)?.less(&Self::interpret(&right)?)?,
                )),
                Token::MoreEqual => Ok(Value::Boolean(
                    Self::interpret(&left)?.more_equal(&Self::interpret(&right)?)?,
                )),
                Token::More => Ok(Value::Boolean(
                    Self::interpret(&left)?.more(&Self::interpret(&right)?)?,
                )),
                _ => Err(TreeWalkerErr::UnsupportedOperator),
            },
            Expr::Literal(Literal::Integer(val)) => Ok(Value::Integer(*val)),
            Expr::Literal(Literal::Float(val)) => Ok(Value::Float(*val)),
            Expr::Literal(Literal::String(val)) => Ok(Value::String(val.clone())),
            Expr::Literal(Literal::Boolean(val)) => Ok(Value::Boolean(val.clone())),
        }
    }
}
