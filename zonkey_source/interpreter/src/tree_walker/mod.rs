use self::{
    err::TreeWalkerErr,
    status::TreeWalkerStatus,
    value::{Value, ValueType},
};
use crate::{environment::Environment, expr::Expr, literal::Literal, stmt::Stmt, token::Token};
use std::slice::Iter;

pub mod err;
pub mod status;
pub mod value;

pub struct TreeWalker<'a> {
    environment: &'a mut Environment,
}

impl<'a> TreeWalker<'a> {
    pub fn new(environment: &'a mut Environment) -> Self {
        Self { environment }
    }

    pub fn run(mut self, statements: Iter<'a, Stmt>) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        for statement in statements {
            match self.interpret(&statement) {
                Ok(TreeWalkerStatus::Ok) => continue,
                Ok(TreeWalkerStatus::Exit) => return Ok(TreeWalkerStatus::Exit),
                Ok(TreeWalkerStatus::Break) => return Err(TreeWalkerErr::BreakOutsideLoop),
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
            Stmt::VariableAssignment(name, expr, operator) => {
                self.variable_assignment(name, expr, operator)
            }
            Stmt::Block(statements) => {
                self.environment.push();

                let mut return_value = Ok(TreeWalkerStatus::Ok);

                for statement in statements {
                    match self.interpret(&statement) {
                        Ok(TreeWalkerStatus::Ok) => continue,
                        Ok(TreeWalkerStatus::Exit) => {
                            return_value = Ok(TreeWalkerStatus::Exit);
                            break;
                        }
                        Ok(TreeWalkerStatus::Break) => {
                            return_value = Ok(TreeWalkerStatus::Break);
                            break;
                        }
                        Err(err) => return Err(err),
                    }
                }

                self.environment.pop();

                return_value
            }
            Stmt::If(condition, true_branch, false_branch) => match self.evaluate(condition)? {
                Value::Boolean(true) => self.interpret(true_branch),
                Value::Boolean(false) => {
                    if let Some(branch) = false_branch {
                        self.interpret(branch)
                    } else {
                        Ok(TreeWalkerStatus::Ok)
                    }
                }
                _ => Err(TreeWalkerErr::IfConditionMustEvaluateToBoolean),
            },
            Stmt::While(condition, block) => {
                while match self.evaluate(condition)? {
                    Value::Boolean(true) => true,
                    Value::Boolean(false) => false,
                    _ => return Err(TreeWalkerErr::IfConditionMustEvaluateToBoolean),
                } {
                    match self.interpret(block)? {
                        TreeWalkerStatus::Break => break,
                        TreeWalkerStatus::Ok => continue,
                        TreeWalkerStatus::Exit => return Ok(TreeWalkerStatus::Exit),
                    }
                }

                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Loop(block) => {
                loop {
                    match self.interpret(block)? {
                        TreeWalkerStatus::Break => break,
                        TreeWalkerStatus::Ok => continue,
                        TreeWalkerStatus::Exit => return Ok(TreeWalkerStatus::Exit),
                    }
                }

                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Break => Ok(TreeWalkerStatus::Break),
            Stmt::Continue => Ok(TreeWalkerStatus::Ok),
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

    fn variable_assignment(
        &mut self,
        name: &String,
        expression: &Expr,
        operator: &Token,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        let variable = match self.environment.get(name) {
            Some(var) => var,
            None => return Err(TreeWalkerErr::VariableNotDefined(name.clone())),
        };
        let variable_type = variable.get_value_type();

        let value = self.evaluate(expression)?;
        let value_type = value.get_value_type();

        if variable_type != value_type {
            return Err(TreeWalkerErr::VariableAssignmentIncompatibleTypes(
                variable_type,
                value_type,
            ));
        }

        self.environment.assign(name, value, operator);

        Ok(TreeWalkerStatus::Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::{value::ValueType, TreeWalker};
    use crate::{
        environment::Environment,
        expr::Expr,
        literal::Literal,
        stmt::Stmt,
        token::Token,
        tree_walker::{err::TreeWalkerErr, value::Value},
    };

    #[test]
    fn test_scope_1() {
        let mut environment = Environment::new();
        let tree_walker = TreeWalker::new(&mut environment);

        // This program tests scope by checking the result variable is 30 at the end of execution.
        // Global variables 'result' and 'a' are initialised to 0 and 2 respectively.
        // A scope is then created with the local variable 'b' initialised to 19.
        // 'result' is then reassigned to the multiplication of a*b inside the local scope (result=38).
        // Finally, 8 is subtracted from result in the global scope (result=30).
        tree_walker
            .run(
                vec![
                    Stmt::VariableDeclaration(
                        ValueType::Integer,
                        String::from("result"),
                        Expr::Literal(Literal::Integer(0)),
                    ),
                    Stmt::VariableDeclaration(
                        ValueType::Integer,
                        String::from("a"),
                        Expr::Literal(Literal::Integer(2)),
                    ),
                    Stmt::Block(vec![
                        Stmt::VariableDeclaration(
                            ValueType::Integer,
                            String::from("b"),
                            Expr::Literal(Literal::Integer(19)),
                        ),
                        Stmt::VariableAssignment(
                            String::from("result"),
                            Expr::Binary {
                                left: Box::new(Expr::Variable(String::from("a"))),
                                operator: Token::Star,
                                right: Box::new(Expr::Variable(String::from("b"))),
                            },
                            Token::Equal,
                        ),
                    ]),
                    Stmt::VariableAssignment(
                        String::from("result"),
                        Expr::Binary {
                            left: Box::new(Expr::Variable(String::from("result"))),
                            operator: Token::Minus,
                            right: Box::new(Expr::Literal(Literal::Integer(8))),
                        },
                        Token::Equal,
                    ),
                ]
                .iter(),
            )
            .expect("Tree walker failed to run test program");

        assert_eq!(
            *environment.get(&String::from("result")).unwrap(),
            Value::Integer(30)
        );
    }

    #[test]
    fn test_scope_2() {
        let mut environment = Environment::new();
        let tree_walker = TreeWalker::new(&mut environment);

        // Try to print a variable that was created in a local scope in the global scope.
        let result = tree_walker.run(
            vec![
                Stmt::Block(vec![Stmt::VariableDeclaration(
                    ValueType::String,
                    String::from("local_variable"),
                    Expr::Literal(Literal::String(String::from("Hello"))),
                )]),
                Stmt::Print(Expr::Variable(String::from("local_variable"))),
            ]
            .iter(),
        );

        assert_eq!(
            result,
            Err(TreeWalkerErr::VariableNotDefined(String::from(
                "local_variable"
            )))
        );
    }
}
