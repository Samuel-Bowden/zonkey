use self::{err::TreeWalkerErr, status::TreeWalkerStatus, value::Value};
use crate::{expr::Expr, literal::Literal, stmt::Stmt, token::Token};
use std::slice::Iter;

pub mod err;
pub mod status;
mod value;

pub struct TreeWalker<'a> {
    statements: &'a mut Iter<'a, Stmt>,
}

impl<'a> TreeWalker<'a> {
    pub fn new(statements: &'a mut Iter<'a, Stmt>) -> Self {
        Self { statements }
    }

    pub fn run(self) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        for statement in self.statements {
            match Self::interpret(&statement) {
                Ok(TreeWalkerStatus::Ok) => continue,
                Ok(TreeWalkerStatus::Exit) => return Ok(TreeWalkerStatus::Exit),
                Err(err) => return Err(err),
            }
        }

        Ok(TreeWalkerStatus::Ok)
    }

    fn interpret(statement: &Stmt) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        match statement {
            Stmt::Print(expr) => {
                println!("{}", Self::evaluate(expr)?);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Expression(expr) => {
                Self::evaluate(expr)?;
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Exit => Ok(TreeWalkerStatus::Exit),
        }
    }

    fn evaluate(expression: &Expr) -> Result<Value, TreeWalkerErr> {
        match expression {
            Expr::Binary {
                left,
                operator,
                right,
            } => match operator {
                Token::Minus => Ok((Self::evaluate(&left)? - Self::evaluate(&right)?)?),
                Token::Plus => Ok((Self::evaluate(&left)? + Self::evaluate(&right)?)?),
                Token::Slash => Ok((Self::evaluate(&left)? / Self::evaluate(&right)?)?),
                Token::Star => Ok((Self::evaluate(&left)? * Self::evaluate(&right)?)?),
                Token::EqualEqual => Ok(Value::Boolean(
                    Self::evaluate(&left)?.equal(&Self::evaluate(&right)?)?,
                )),
                Token::BangEqual => Ok(Value::Boolean(
                    !(Self::evaluate(&left)?.equal(&Self::evaluate(&right)?)?),
                )),
                Token::LessEqual => Ok(Value::Boolean(
                    Self::evaluate(&left)?.less_equal(&Self::evaluate(&right)?)?,
                )),
                Token::Less => Ok(Value::Boolean(
                    Self::evaluate(&left)?.less(&Self::evaluate(&right)?)?,
                )),
                Token::MoreEqual => Ok(Value::Boolean(
                    Self::evaluate(&left)?.more_equal(&Self::evaluate(&right)?)?,
                )),
                Token::More => Ok(Value::Boolean(
                    Self::evaluate(&left)?.more(&Self::evaluate(&right)?)?,
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
