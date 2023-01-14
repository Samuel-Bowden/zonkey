use self::{
    err::TreeWalkerErr,
    status::TreeWalkerStatus,
    value::{Value, ValueType},
};
use crate::{
    environment::Environment, expr::Expr, global::Global, literal::Literal, stmt::Stmt,
    token::Token,
};

pub mod err;
pub mod status;
pub mod value;

pub struct TreeWalker<'a> {
    environment: &'a mut Environment,
    global: &'a Global,
}

impl<'a> TreeWalker<'a> {
    pub fn new(environment: &'a mut Environment, global: &'a Global) -> Self {
        Self {
            environment,
            global,
        }
    }

    pub fn interpret(&mut self, statement: &Stmt) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        match statement {
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
            Stmt::FunctionDeclaration(_, _, _) => Err(TreeWalkerErr::NestedFunctionsNotAllowed),
            Stmt::Start(_) => Err(TreeWalkerErr::StartNotInGlobalScope),
        }
    }

    fn evaluate(&self, expression: &Expr) -> Result<Value, TreeWalkerErr> {
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
            Expr::Literal(Literal::Boolean(val)) => Ok(Value::Boolean(*val)),
            Expr::Variable(name) => match self.environment.get(&name) {
                Some(value) => Ok(value.clone()),
                None => Err(TreeWalkerErr::VariableNotDefined(name.clone())),
            },
            Expr::Call(name, arguments) => match self.global.get_function(name) {
                Some(function) => {
                    let mut argument_values = vec![];

                    for argument in arguments {
                        argument_values.push(self.evaluate(argument)?);
                    }

                    function.call(&argument_values, self.global)?;

                    Ok(Value::Boolean(true))
                }
                None => Err(TreeWalkerErr::FunctionNotDefined(name.clone())),
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
        let variable = match self.environment.get(&name) {
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

        self.environment
            .assign(name.clone(), value, operator.clone());

        Ok(TreeWalkerStatus::Ok)
    }
}
