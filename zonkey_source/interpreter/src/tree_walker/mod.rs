use numtoa::NumToA;

use self::{environment::Environment, status::TreeWalkerStatus};
use crate::{
    ast::AST,
    err::tree_walker::TreeWalkerErr,
    event::Event,
    expr::*,
    prelude::*,
    stmt::{ConstructionType, Stmt},
};
use std::{
    io::{stdout, BufWriter, StdoutLock, Write},
    sync::mpsc::Sender,
};

mod environment;
pub mod status;

pub struct TreeWalker<'a> {
    environment: Environment,
    callables: &'a Vec<Stmt>,
    stdout: BufWriter<StdoutLock<'a>>,
    sender: Sender<Event>,
}

impl<'a> TreeWalker<'a> {
    pub fn run(ast: AST, sender: Sender<Event>) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        let environment = Environment::new();
        TreeWalker::new(&ast.callable, environment, sender).interpret(&ast.start)
    }

    pub fn new(callables: &'a Vec<Stmt>, environment: Environment, sender: Sender<Event>) -> Self {
        Self {
            environment,
            callables,
            stdout: BufWriter::new(stdout().lock()),
            sender,
        }
    }

    pub fn interpret(&mut self, statement: &Stmt) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        match statement {
            Stmt::IntegerVariableInitialisation(expr) => {
                let int = self.eval_int(expr)?;
                self.environment.push_int(int);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::IntegerVariableAssignment(id, expr, assignment_operator) => {
                let int = self.eval_int(expr)?;
                self.environment.assign_int(*id, int, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::FloatVariableInitialisation(expr) => {
                let float = self.eval_float(expr)?;
                self.environment.push_float(float);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::FloatVariableAssignment(id, expr, assignment_operator) => {
                let float = self.eval_float(expr)?;
                self.environment
                    .assign_float(*id, float, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::StringVariableInitialisation(expr) => {
                let string = self.eval_string(expr)?;
                self.environment.push_string(string);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::ClassVariableInitialisation(types) => {
                self.construct_class(types);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::StringVariableAssignment(id, expr, assignment_operator) => {
                let string = self.eval_string(expr)?;
                self.environment
                    .assign_string(*id, string, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::BooleanVariableInitialisation(expr) => {
                let boolean = self.eval_boolean(expr)?;
                self.environment.push_boolean(boolean);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::BooleanVariableAssignment(id, expr, assignment_operator) => {
                let boolean = self.eval_boolean(expr)?;
                self.environment
                    .assign_boolean(*id, boolean, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Block(statements, block_start_points) => {
                self.environment.push_stack();

                let mut return_value = Ok(TreeWalkerStatus::Ok);

                for statement in statements {
                    match self.interpret(statement) {
                        Ok(TreeWalkerStatus::Ok) => (),
                        other => {
                            return_value = other;
                            break;
                        }
                    }
                }

                self.environment.pop_stack(block_start_points);

                return_value
            }
            Stmt::If(condition, true_branch, false_branch) => {
                if self.eval_boolean(condition)? {
                    self.interpret(&true_branch)
                } else if let Some(false_branch) = false_branch {
                    self.interpret(&false_branch)
                } else {
                    Ok(TreeWalkerStatus::Ok)
                }
            }
            Stmt::Expression(expr) => {
                match expr {
                    Expr::Integer(expr) => {
                        self.eval_int(expr)?;
                    }
                    Expr::Float(expr) => {
                        self.eval_float(expr)?;
                    }
                    Expr::String(expr) => {
                        self.eval_string(expr)?;
                    }
                    Expr::Boolean(expr) => {
                        self.eval_boolean(expr)?;
                    }
                    Expr::None(expr) => {
                        self.eval_none(expr)?;
                    }
                    Expr::Object(..) => {}
                }

                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::While(condition, block) => {
                while self.eval_boolean(condition)? {
                    match self.interpret(block) {
                        Ok(TreeWalkerStatus::Ok) => (),
                        Ok(TreeWalkerStatus::Continue) => (),
                        Ok(TreeWalkerStatus::Break) => break,
                        other => return other,
                    }
                }

                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Loop(block) => {
                loop {
                    match self.interpret(block) {
                        Ok(TreeWalkerStatus::Ok) => (),
                        Ok(TreeWalkerStatus::Continue) => (),
                        Ok(TreeWalkerStatus::Break) => break,
                        other => return other,
                    }
                }

                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Break => Ok(TreeWalkerStatus::Break),
            Stmt::Continue => Ok(TreeWalkerStatus::Continue),
            Stmt::Return(expr) => match expr {
                Some(Expr::Integer(expr)) => Ok(TreeWalkerStatus::ReturnInt(self.eval_int(expr)?)),
                Some(Expr::Float(expr)) => {
                    Ok(TreeWalkerStatus::ReturnFloat(self.eval_float(expr)?))
                }
                Some(Expr::String(expr)) => {
                    Ok(TreeWalkerStatus::ReturnString(self.eval_string(expr)?))
                }
                Some(Expr::Boolean(expr)) => {
                    Ok(TreeWalkerStatus::ReturnBoolean(self.eval_boolean(expr)?))
                }
                Some(Expr::None(expr)) => {
                    self.eval_none(expr)?;
                    Ok(TreeWalkerStatus::ReturnNone)
                }
                Some(Expr::Object(..)) => {
                    todo!()
                }
                None => Ok(TreeWalkerStatus::ReturnNone),
            },
        }
    }

    fn eval_int(&mut self, expression: &IntegerExpr) -> Result<i64, TreeWalkerErr> {
        match expression {
            IntegerExpr::Binary {
                left,
                operator,
                right,
            } => match operator {
                NumericOperator::Add => Ok(self.eval_int(left)? + self.eval_int(right)?),
                NumericOperator::Subtract => Ok(self.eval_int(left)? - self.eval_int(right)?),
                NumericOperator::Multiply => Ok(self.eval_int(left)? * self.eval_int(right)?),
                NumericOperator::Divide => {
                    let left = self.eval_int(left)?;
                    let right = self.eval_int(right)?;

                    if right == 0 {
                        Err(TreeWalkerErr::DivisionByZero)
                    } else {
                        Ok(left / right)
                    }
                }
            },
            IntegerExpr::Unary(unary_operator, expr) => match unary_operator {
                NumericUnaryOperator::Minus => Ok(-self.eval_int(expr)?),
            },
            IntegerExpr::Variable(id) => Ok(self.environment.get_int(*id)),
            IntegerExpr::Literal(val) => Ok(*val),
            IntegerExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnInt(v) => Ok(v),
                _ => panic!("Call did not return correct type"),
            },
            IntegerExpr::FloatCast(expr) => Ok(self.eval_float(expr)? as i64),
            IntegerExpr::BooleanCast(expr) => Ok(self.eval_boolean(expr)? as i64),
            IntegerExpr::StringCast(expr) => match self.eval_string(expr)?.parse() {
                Err(_) => Err(TreeWalkerErr::FailedStringToIntegerCast),
                Ok(val) => Ok(val),
            },
        }
    }

    fn eval_float(&mut self, expression: &FloatExpr) -> Result<f64, TreeWalkerErr> {
        match expression {
            FloatExpr::Binary {
                left,
                operator,
                right,
            } => match operator {
                NumericOperator::Add => Ok(self.eval_float(left)? + self.eval_float(right)?),
                NumericOperator::Subtract => Ok(self.eval_float(left)? - self.eval_float(right)?),
                NumericOperator::Multiply => Ok(self.eval_float(left)? * self.eval_float(right)?),
                NumericOperator::Divide => Ok(self.eval_float(left)? / self.eval_float(right)?),
            },
            FloatExpr::Unary(unary_operator, expr) => match unary_operator {
                NumericUnaryOperator::Minus => Ok(-self.eval_float(expr)?),
            },
            FloatExpr::Variable(id) => Ok(self.environment.get_float(*id)),
            FloatExpr::Literal(val) => Ok(*val),
            FloatExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnFloat(v) => Ok(v),
                _ => panic!("Call did not return correct type"),
            },
            FloatExpr::IntegerCast(expr) => Ok(self.eval_int(expr)? as f64),
            FloatExpr::BooleanCast(expr) => Ok(self.eval_boolean(expr)? as i64 as f64),
            FloatExpr::StringCast(expr) => match self.eval_string(expr)?.parse() {
                Err(_) => Err(TreeWalkerErr::FailedStringToFloatCast),
                Ok(val) => Ok(val),
            },
        }
    }

    fn eval_string(&mut self, expression: &StringExpr) -> Result<String, TreeWalkerErr> {
        match expression {
            StringExpr::Binary {
                left,
                operator,
                right,
            } => match operator {
                StringOperator::Add => Ok(self.eval_string(left)? + &self.eval_string(right)?),
            },
            StringExpr::Variable(id) => Ok(self.environment.get_string(*id)),
            StringExpr::Literal(val) => Ok(val.to_string()),
            StringExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnString(v) => Ok(v),
                _ => panic!("Call did not return correct type"),
            },
            StringExpr::IntegerCast(expr) => Ok(self.eval_int(expr)?.to_string()),
            StringExpr::FloatCast(expr) => Ok(self.eval_float(expr)?.to_string()),
            StringExpr::BooleanCast(expr) => Ok(self.eval_boolean(expr)?.to_string()),
            StringExpr::NativeCall(call) => self.native_call_string(call),
        }
    }

    fn eval_boolean(&mut self, expression: &BooleanExpr) -> Result<bool, TreeWalkerErr> {
        match expression {
            BooleanExpr::IntegerBinary {
                left,
                comparator,
                right,
            } => match comparator {
                NumericComparision::Equal => Ok(self.eval_int(left)? == self.eval_int(right)?),
                NumericComparision::Inequal => Ok(self.eval_int(left)? != self.eval_int(right)?),
                NumericComparision::MoreEqual => Ok(self.eval_int(left)? >= self.eval_int(right)?),
                NumericComparision::LessEqual => Ok(self.eval_int(left)? <= self.eval_int(right)?),
                NumericComparision::More => Ok(self.eval_int(left)? > self.eval_int(right)?),
                NumericComparision::Less => Ok(self.eval_int(left)? < self.eval_int(right)?),
            },
            BooleanExpr::FloatBinary {
                left,
                comparator,
                right,
            } => match comparator {
                NumericComparision::Equal => Ok(self.eval_float(left)? == self.eval_float(right)?),
                NumericComparision::Inequal => {
                    Ok(self.eval_float(left)? != self.eval_float(right)?)
                }
                NumericComparision::MoreEqual => {
                    Ok(self.eval_float(left)? >= self.eval_float(right)?)
                }
                NumericComparision::LessEqual => {
                    Ok(self.eval_float(left)? <= self.eval_float(right)?)
                }
                NumericComparision::More => Ok(self.eval_float(left)? > self.eval_float(right)?),
                NumericComparision::Less => Ok(self.eval_float(left)? < self.eval_float(right)?),
            },
            BooleanExpr::StringBinary {
                left,
                comparator,
                right,
            } => match comparator {
                StringComparision::Equal => Ok(self.eval_string(left)? == self.eval_string(right)?),
                StringComparision::Inequal => {
                    Ok(self.eval_string(left)? != self.eval_string(right)?)
                }
            },
            BooleanExpr::BooleanBinary {
                left,
                comparator,
                right,
            } => match comparator {
                BooleanComparision::Equal => {
                    Ok(self.eval_boolean(left)? == self.eval_boolean(right)?)
                }
                BooleanComparision::Inequal => {
                    Ok(self.eval_boolean(left)? != self.eval_boolean(right)?)
                }
                BooleanComparision::And => {
                    Ok(self.eval_boolean(left)? && self.eval_boolean(right)?)
                }
                BooleanComparision::Or => Ok(self.eval_boolean(left)? || self.eval_boolean(right)?),
            },
            BooleanExpr::Variable(id) => Ok(self.environment.get_boolean(*id)),
            BooleanExpr::Literal(val) => Ok(*val),
            BooleanExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnBoolean(v) => Ok(v),
                _ => panic!("Call did not return correct type"),
            },
            BooleanExpr::Unary(unary_operator, expr) => match unary_operator {
                BooleanUnaryOperator::Bang => Ok(!self.eval_boolean(expr)?),
            },
            BooleanExpr::IntegerCast(expr) => Ok(self.eval_int(expr)? != 0),
            BooleanExpr::FloatCast(expr) => Ok(self.eval_float(expr)? != 0.),
            BooleanExpr::StringCast(expr) => match self.eval_string(expr)?.parse() {
                Err(_) => Err(TreeWalkerErr::FailedStringToBooleanCast),
                Ok(val) => Ok(val),
            },
        }
    }

    fn eval_none(&mut self, expression: &NoneExpr) -> Result<(), TreeWalkerErr> {
        match expression {
            NoneExpr::NativeCall(call) => self.native_call_none(call),
            NoneExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnNone | TreeWalkerStatus::Ok => Ok(()),
                _ => panic!("Call did not return correct type"),
            },
        }
    }

    fn native_call_none(&mut self, call: &NativeFunctionNone) -> Result<(), TreeWalkerErr> {
        match call {
            NativeFunctionNone::Print(expr, line) => match &**expr {
                Expr::Integer(expr) => {
                    let mut buffer = [0u8; 20];
                    let int = self.eval_int(expr)?.numtoa(10, &mut buffer);
                    self.stdout.write(int).unwrap();
                    if *line {
                        self.stdout.write(b"\n").unwrap();
                    }
                }
                Expr::Float(expr) => {
                    let mut buffer = ryu::Buffer::new();
                    let float = buffer.format(self.eval_float(expr)?).as_bytes();
                    self.stdout.write(float).unwrap();
                    if *line {
                        self.stdout.write(b"\n").unwrap();
                    }
                }
                Expr::String(expr) => {
                    let string = self.eval_string(&expr)?;
                    write!(self.stdout, "{}{}", string, if *line { "\n" } else { "" }).unwrap();
                }
                Expr::Boolean(expr) => {
                    let boolean = self.eval_boolean(expr)?;
                    write!(self.stdout, "{}{}", boolean, if *line { "\n" } else { "" }).unwrap();
                }
                _ => panic!("Unprintable type"),
            },
        }

        Ok(())
    }

    fn native_call_string(&mut self, call: &NativeFunctionString) -> Result<String, TreeWalkerErr> {
        match call {
            NativeFunctionString::Prompt(expr) => {
                let prompt = self.eval_string(expr)?;

                self.stdout.flush().unwrap();

                write!(self.stdout, "{prompt} ").unwrap();

                self.stdout.flush().unwrap();

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                Ok(input.trim().to_string())
            }
        }
    }

    fn eval_call(
        &mut self,
        id: usize,
        expressions: &Vec<Expr>,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        let callable = &self.callables[id];

        let mut environment = Environment::new();

        for expression in expressions {
            self.add_expr_to_environment(expression, &mut environment)?;
        }

        std::mem::swap(&mut environment, &mut self.environment);

        let result = self.interpret(&callable);

        std::mem::swap(&mut environment, &mut self.environment);

        result
    }

    fn add_expr_to_environment(
        &mut self,
        expr: &Expr,
        environment: &mut Environment,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        match expr {
            Expr::Integer(expr) => environment.push_int(self.eval_int(expr)?),
            Expr::Float(expr) => environment.push_float(self.eval_float(expr)?),
            Expr::String(expr) => environment.push_string(self.eval_string(expr)?),
            Expr::Boolean(expr) => environment.push_boolean(self.eval_boolean(expr)?),
            Expr::None(_) => panic!("Cannot pass none to a callable"),
            Expr::Object(_, _, exprs) => {
                for expr in exprs {
                    self.add_expr_to_environment(expr, environment)?;
                }
            }
        }

        Ok(TreeWalkerStatus::Ok)
    }

    fn construct_class(&mut self, types: &Vec<ConstructionType>) {
        for value_type in types {
            match value_type {
                ConstructionType::Integer => self.environment.push_int(0),
                ConstructionType::Float => self.environment.push_float(0.),
                ConstructionType::String => self.environment.push_string(String::new()),
                ConstructionType::Boolean => self.environment.push_boolean(false),
                ConstructionType::Class(types) => self.construct_class(types),
            }
        }
    }

    fn event(&mut self, event: Event) -> Result<(), TreeWalkerErr> {
        match self.sender.send(event) {
            Ok(()) => Ok(()),
            Err(_) => Err(TreeWalkerErr::FailedToSendEventToBrowser),
        }
    }
}
