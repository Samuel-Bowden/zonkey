use self::{
    err::TreeWalkerErr,
    status::TreeWalkerStatus,
    value::{Value, ValueType},
};
use crate::{expr::Expr, literal::Literal, stmt::Stmt, token::Token};
use std::{collections::HashMap, slice::Iter};

pub mod err;
pub mod status;
pub mod value;

pub struct TreeWalker<'a> {
    environment: &'a mut HashMap<String, Value>,
}

impl<'a> TreeWalker<'a> {
    pub fn new(environment: &'a mut HashMap<String, Value>) -> Self {
        Self { environment }
    }

    pub fn run(mut self, statements: Iter<'a, Stmt>) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        for statement in statements {
            match self.interpret(&statement) {
                Ok(TreeWalkerStatus::Ok) => continue,
                Ok(TreeWalkerStatus::Exit) => return Ok(TreeWalkerStatus::Exit),
                Err(err) => return Err(err),
            }
        }

        Ok(TreeWalkerStatus::Ok)
    }

    fn interpret(&mut self, statement: &Stmt) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        match statement {
            Stmt::Print(expr) => {
                println!("{}", self.evaluate(expr)?);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Exit => Ok(TreeWalkerStatus::Exit),
            Stmt::VariableDeclaration(data_type, name, expr) => {
                self.variable_declaration(data_type, name, expr)
            }
        }
    }

    fn evaluate(&mut self, expression: &Expr) -> Result<Value, TreeWalkerErr> {
        match expression {
            Expr::Binary {
                left,
                operator,
                right,
            } => match operator {
                Token::Minus => Ok((self.evaluate(&left)? - self.evaluate(&right)?)?),
                Token::Plus => Ok((self.evaluate(&left)? + self.evaluate(&right)?)?),
                Token::Slash => Ok((self.evaluate(&left)? / self.evaluate(&right)?)?),
                Token::Star => Ok((self.evaluate(&left)? * self.evaluate(&right)?)?),
                Token::EqualEqual => Ok(Value::Boolean(
                    self.evaluate(&left)?.equal(&self.evaluate(&right)?)?,
                )),
                Token::BangEqual => Ok(Value::Boolean(
                    !(self.evaluate(&left)?.equal(&self.evaluate(&right)?)?),
                )),
                Token::LessEqual => Ok(Value::Boolean(
                    self.evaluate(&left)?.less_equal(&self.evaluate(&right)?)?,
                )),
                Token::Less => Ok(Value::Boolean(
                    self.evaluate(&left)?.less(&self.evaluate(&right)?)?,
                )),
                Token::MoreEqual => Ok(Value::Boolean(
                    self.evaluate(&left)?.more_equal(&self.evaluate(&right)?)?,
                )),
                Token::More => Ok(Value::Boolean(
                    self.evaluate(&left)?.more(&self.evaluate(&right)?)?,
                )),
                _ => Err(TreeWalkerErr::UnsupportedOperator),
            },
            Expr::Literal(Literal::Integer(val)) => Ok(Value::Integer(*val)),
            Expr::Literal(Literal::Float(val)) => Ok(Value::Float(*val)),
            Expr::Literal(Literal::String(val)) => Ok(Value::String(val.clone())),
            Expr::Literal(Literal::Boolean(val)) => Ok(Value::Boolean(val.clone())),
            Expr::Variable(name) => match self.environment.get(name) {
                Some(value) => Ok(value.clone()),
                None => return Err(TreeWalkerErr::VariableNotDefined(name.clone())),
            },
        }
    }

    fn variable_declaration(
        &mut self,
        data_type: &ValueType,
        name: &String,
        expression: &Expr,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        let value = self.evaluate(expression)?;

        let value_data_type = value.get_value_type();

        if *data_type != value_data_type {
            return Err(TreeWalkerErr::VariableAssignmentIncompatibleTypes(
                data_type.clone(),
                value_data_type,
            ));
        }

        self.environment.insert(name.clone(), value);

        Ok(TreeWalkerStatus::Ok)
    }
}
