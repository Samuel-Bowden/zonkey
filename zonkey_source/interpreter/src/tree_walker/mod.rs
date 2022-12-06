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
                        Ok(Self::interpret(&left)? - Self::interpret(&right)?)
                    }
                    TokenType::Plus => {
                        Ok(Self::interpret(&right)? + Self::interpret(&left)?)
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
            _ => Err(TreeWalkerErr::UnsupportedExpression)
        }
    }
}
