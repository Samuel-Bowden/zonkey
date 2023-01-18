use self::{err::TreeWalkerErr, status::TreeWalkerStatus};
use crate::{
    comparison::{BooleanComparision, NumericComparision, StringComparision},
    environment::Environment,
    expr::{BooleanExpr, Expr, FloatExpr, IntegerExpr, NoneExpr, StringExpr},
    function::Function,
    native_function::{
        cli_api::{prompt::{prompt, prompt_int}, CliFunctionNone, CliFunctionString, CliFunctionInteger},
        NativeFunctionNone, NativeFunctionString, NativeFunctionInteger,
    },
    operator::{NumericOperator, StringOperator},
    stmt::Stmt,
};

pub mod err;
pub mod status;

pub struct TreeWalker<'a> {
    environment: Environment,
    functions: &'a Vec<Function>,
}

impl<'a> TreeWalker<'a> {
    pub fn new(functions: &'a Vec<Function>, environment: Environment) -> Self {
        Self {
            environment,
            functions,
        }
    }

    pub fn interpret(&mut self, statement: &Stmt) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        match statement {
            Stmt::IntegerVariableDeclaration(expr) => {
                self.environment.push_int(self.eval_int(expr));
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::IntegerVariableAssignment(id, expr, assignment_operator) => {
                self.environment
                    .assign_int(*id, self.eval_int(expr), assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::FloatVariableDeclaration(expr) => {
                self.environment.push_float(self.eval_float(expr));
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::FloatVariableAssignment(id, expr, assignment_operator) => {
                self.environment
                    .assign_float(*id, self.eval_float(expr), assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::StringVariableDeclaration(expr) => {
                self.environment.push_string(self.eval_string(expr));
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::StringVariableAssignment(id, expr, assignment_operator) => {
                self.environment
                    .assign_string(*id, self.eval_string(expr), assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::BooleanVariableDeclaration(expr) => {
                self.environment.push_boolean(self.eval_boolean(expr));
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::BooleanVariableAssignment(id, expr, assignment_operator) => {
                self.environment
                    .assign_boolean(*id, self.eval_boolean(expr), assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Block(statements, block_start_points) => {
                self.environment.push_stack();

                let mut return_value = Ok(TreeWalkerStatus::Ok);

                for statement in statements {
                    match self.interpret(statement)? {
                        TreeWalkerStatus::Ok => (),
                        TreeWalkerStatus::Continue => {
                            return_value = Ok(TreeWalkerStatus::Continue);
                            break;
                        }
                        TreeWalkerStatus::Break => {
                            return_value = Ok(TreeWalkerStatus::Break);
                            break;
                        }
                        TreeWalkerStatus::ReturnInt(v) => {
                            return_value = Ok(TreeWalkerStatus::ReturnInt(v));
                            break;
                        }
                        TreeWalkerStatus::ReturnFloat(v) => {
                            return_value = Ok(TreeWalkerStatus::ReturnFloat(v));
                            break;
                        }
                        TreeWalkerStatus::ReturnString(v) => {
                            return_value = Ok(TreeWalkerStatus::ReturnString(v));
                            break;
                        }
                        TreeWalkerStatus::ReturnBoolean(v) => {
                            return_value = Ok(TreeWalkerStatus::ReturnBoolean(v));
                            break;
                        }
                        TreeWalkerStatus::ReturnNone => {
                            return_value = Ok(TreeWalkerStatus::ReturnNone);
                            break;
                        }
                    }
                }

                self.environment.pop_stack(block_start_points);

                return_value
            }
            Stmt::If(condition, true_branch, false_branch) => {
                if self.eval_boolean(condition) {
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
                        self.eval_int(expr);
                    }
                    Expr::Float(expr) => {
                        self.eval_float(expr);
                    }
                    Expr::String(expr) => {
                        self.eval_string(expr);
                    }
                    Expr::Boolean(expr) => {
                        self.eval_boolean(expr);
                    }
                    Expr::None(expr) => {
                        self.eval_none(expr);
                    }
                }

                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::While(condition, block) => {
                while self.eval_boolean(condition) {
                    match self.interpret(block)? {
                        TreeWalkerStatus::Ok => (),
                        TreeWalkerStatus::Continue => (),
                        TreeWalkerStatus::Break => break,
                        TreeWalkerStatus::ReturnInt(v) => {
                            return Ok(TreeWalkerStatus::ReturnInt(v))
                        }
                        TreeWalkerStatus::ReturnFloat(v) => {
                            return Ok(TreeWalkerStatus::ReturnFloat(v))
                        }
                        TreeWalkerStatus::ReturnString(v) => {
                            return Ok(TreeWalkerStatus::ReturnString(v))
                        }
                        TreeWalkerStatus::ReturnBoolean(v) => {
                            return Ok(TreeWalkerStatus::ReturnBoolean(v))
                        }
                        TreeWalkerStatus::ReturnNone => return Ok(TreeWalkerStatus::ReturnNone),
                    }
                }

                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Loop(block) => {
                loop {
                    match self.interpret(block)? {
                        TreeWalkerStatus::Ok => (),
                        TreeWalkerStatus::Continue => (),
                        TreeWalkerStatus::Break => break,
                        TreeWalkerStatus::ReturnInt(v) => {
                            return Ok(TreeWalkerStatus::ReturnInt(v))
                        }
                        TreeWalkerStatus::ReturnFloat(v) => {
                            return Ok(TreeWalkerStatus::ReturnFloat(v))
                        }
                        TreeWalkerStatus::ReturnString(v) => {
                            return Ok(TreeWalkerStatus::ReturnString(v))
                        }
                        TreeWalkerStatus::ReturnBoolean(v) => {
                            return Ok(TreeWalkerStatus::ReturnBoolean(v))
                        }
                        TreeWalkerStatus::ReturnNone => return Ok(TreeWalkerStatus::ReturnNone),
                    }
                }

                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::Break => Ok(TreeWalkerStatus::Break),
            Stmt::Continue => Ok(TreeWalkerStatus::Continue),
            Stmt::Return(expr) => match expr {
                Some(Expr::Integer(expr)) => Ok(TreeWalkerStatus::ReturnInt(self.eval_int(expr))),
                Some(Expr::Float(expr)) => Ok(TreeWalkerStatus::ReturnFloat(self.eval_float(expr))),
                Some(Expr::String(expr)) => {
                    Ok(TreeWalkerStatus::ReturnString(self.eval_string(expr)))
                }
                Some(Expr::Boolean(expr)) => {
                    Ok(TreeWalkerStatus::ReturnBoolean(self.eval_boolean(expr)))
                }
                Some(Expr::None(expr)) => {
                    self.eval_none(expr);
                    Ok(TreeWalkerStatus::ReturnNone)
                }
                None => Ok(TreeWalkerStatus::ReturnNone),
            },
        }
    }

    fn eval_int(&self, expression: &IntegerExpr) -> i64 {
        match expression {
            IntegerExpr::Binary {
                left,
                operator,
                right,
            } => match operator {
                NumericOperator::Add => self.eval_int(left) + self.eval_int(right),
                NumericOperator::Subtract => self.eval_int(left) - self.eval_int(right),
                NumericOperator::Multiply => self.eval_int(left) * self.eval_int(right),
                NumericOperator::Divide => self.eval_int(left) / self.eval_int(right),
            },
            IntegerExpr::Variable(id) => self.environment.get_int(*id),
            IntegerExpr::Literal(val) => *val,
            IntegerExpr::Call(id, expressions) => {
                let function = &self.functions[*id];

                let mut environment = Environment::new();

                for expression in expressions {
                    match expression {
                        Expr::Integer(expr) => environment.push_int(self.eval_int(expr)),
                        Expr::Float(expr) => environment.push_float(self.eval_float(expr)),
                        Expr::String(expr) => environment.push_string(self.eval_string(expr)),
                        Expr::Boolean(expr) => environment.push_boolean(self.eval_boolean(expr)),
                        Expr::None(_) => panic!("Cannot pass none to a function"),
                    }
                }

                match TreeWalker::new(self.functions, environment).interpret(&function.start) {
                    Ok(TreeWalkerStatus::ReturnInt(v)) => v,
                    _ => panic!("Function did not return the correct type"),
                }
            }
            IntegerExpr::NativeCall(call) => self.native_call_integer(call),
        }
    }

    fn eval_float(&self, expression: &FloatExpr) -> f64 {
        match expression {
            FloatExpr::Binary {
                left,
                operator,
                right,
            } => match operator {
                NumericOperator::Add => self.eval_float(left) + self.eval_float(right),
                NumericOperator::Subtract => self.eval_float(left) - self.eval_float(right),
                NumericOperator::Multiply => self.eval_float(left) * self.eval_float(right),
                NumericOperator::Divide => self.eval_float(left) / self.eval_float(right),
            },
            FloatExpr::Variable(id) => self.environment.get_float(*id),
            FloatExpr::Literal(val) => *val,
            FloatExpr::Call(id, expressions) => {
                let function = &self.functions[*id];

                let mut environment = Environment::new();

                for expression in expressions {
                    match expression {
                        Expr::Integer(expr) => environment.push_int(self.eval_int(expr)),
                        Expr::Float(expr) => environment.push_float(self.eval_float(expr)),
                        Expr::String(expr) => environment.push_string(self.eval_string(expr)),
                        Expr::Boolean(expr) => environment.push_boolean(self.eval_boolean(expr)),
                        Expr::None(_) => panic!("Cannot pass none to a function"),
                    }
                }

                match TreeWalker::new(self.functions, environment).interpret(&function.start) {
                    Ok(TreeWalkerStatus::ReturnFloat(v)) => v,
                    _ => panic!("Function did not return the correct type"),
                }
            }
        }
    }

    fn eval_string(&self, expression: &StringExpr) -> String {
        match expression {
            StringExpr::Binary {
                left,
                operator,
                right,
            } => match operator {
                StringOperator::Add => self.eval_string(left) + &self.eval_string(right),
            },
            StringExpr::Variable(id) => self.environment.get_string(*id),
            StringExpr::Literal(val) => val.clone(),
            StringExpr::NativeCall(call) => self.native_call_string(call),
            StringExpr::Call(id, expressions) => {
                let function = &self.functions[*id];

                let mut environment = Environment::new();

                for expression in expressions {
                    match expression {
                        Expr::Integer(expr) => environment.push_int(self.eval_int(expr)),
                        Expr::Float(expr) => environment.push_float(self.eval_float(expr)),
                        Expr::String(expr) => environment.push_string(self.eval_string(expr)),
                        Expr::Boolean(expr) => environment.push_boolean(self.eval_boolean(expr)),
                        Expr::None(_) => panic!("Cannot pass none to a function"),
                    }
                }

                match TreeWalker::new(self.functions, environment).interpret(&function.start) {
                    Ok(TreeWalkerStatus::ReturnString(v)) => v,
                    _ => panic!("Function did not return the correct type"),
                }
            }
        }
    }

    fn eval_boolean(&self, expression: &BooleanExpr) -> bool {
        match expression {
            BooleanExpr::IntegerBinary {
                left,
                comparator,
                right,
            } => match comparator {
                NumericComparision::Equal => self.eval_int(left) == self.eval_int(right),
                NumericComparision::Inequal => self.eval_int(left) != self.eval_int(right),
                NumericComparision::MoreEqual => self.eval_int(left) >= self.eval_int(right),
                NumericComparision::LessEqual => self.eval_int(left) <= self.eval_int(right),
                NumericComparision::More => self.eval_int(left) > self.eval_int(right),
                NumericComparision::Less => self.eval_int(left) < self.eval_int(right),
            },
            BooleanExpr::FloatBinary {
                left,
                comparator,
                right,
            } => match comparator {
                NumericComparision::Equal => self.eval_float(left) == self.eval_float(right),
                NumericComparision::Inequal => self.eval_float(left) != self.eval_float(right),
                NumericComparision::MoreEqual => self.eval_float(left) <= self.eval_float(right),
                NumericComparision::LessEqual => self.eval_float(left) >= self.eval_float(right),
                NumericComparision::More => self.eval_float(left) < self.eval_float(right),
                NumericComparision::Less => self.eval_float(left) > self.eval_float(right),
            },
            BooleanExpr::StringBinary {
                left,
                comparator,
                right,
            } => match comparator {
                StringComparision::Equal => self.eval_string(left) == self.eval_string(right),
                StringComparision::Inequal => self.eval_string(left) != self.eval_string(right),
            },
            BooleanExpr::BooleanBinary {
                left,
                comparator,
                right,
            } => match comparator {
                BooleanComparision::Equal => self.eval_boolean(left) == self.eval_boolean(right),
                BooleanComparision::Inequal => self.eval_boolean(left) != self.eval_boolean(right),
            },
            BooleanExpr::Variable(id) => self.environment.get_boolean(*id),
            BooleanExpr::Literal(val) => *val,
            BooleanExpr::Call(id, expressions) => {
                let function = &self.functions[*id];

                let mut environment = Environment::new();

                for expression in expressions {
                    match expression {
                        Expr::Integer(expr) => environment.push_int(self.eval_int(expr)),
                        Expr::Float(expr) => environment.push_float(self.eval_float(expr)),
                        Expr::String(expr) => environment.push_string(self.eval_string(expr)),
                        Expr::Boolean(expr) => environment.push_boolean(self.eval_boolean(expr)),
                        Expr::None(_) => panic!("Cannot pass none to a function"),
                    }
                }

                match TreeWalker::new(self.functions, environment).interpret(&function.start) {
                    Ok(TreeWalkerStatus::ReturnBoolean(v)) => v,
                    _ => panic!("Function did not return the correct type"),
                }
            }
        }
    }

    fn eval_none(&self, expression: &NoneExpr) {
        match expression {
            NoneExpr::NativeCall(call) => self.native_call_none(call),
            NoneExpr::Call(id, expressions) => {
                let function = &self.functions[*id];

                let mut environment = Environment::new();

                for expression in expressions {
                    match expression {
                        Expr::Integer(expr) => environment.push_int(self.eval_int(expr)),
                        Expr::Float(expr) => environment.push_float(self.eval_float(expr)),
                        Expr::String(expr) => environment.push_string(self.eval_string(expr)),
                        Expr::Boolean(expr) => environment.push_boolean(self.eval_boolean(expr)),
                        Expr::None(_) => panic!("Cannot pass none to a function"),
                    }
                }

                match TreeWalker::new(self.functions, environment).interpret(&function.start) {
                    Ok(TreeWalkerStatus::ReturnNone) | Ok(TreeWalkerStatus::Ok) => (),
                    _ => panic!("Function did not return the correct type"),
                }
            }
        }
    }

    fn native_call_none(&self, call: &NativeFunctionNone) {
        match call {
            NativeFunctionNone::Cli(call) => self.cli_function_none(call),
        }
    }

    fn native_call_integer(&self, call: &NativeFunctionInteger) -> i64 {
        match call {
            NativeFunctionInteger::Cli(call) => self.cli_function_integer(call),
        }
    }

    fn native_call_string(&self, call: &NativeFunctionString) -> String {
        match call {
            NativeFunctionString::Cli(call) => self.cli_function_string(call),
        }
    }

    fn cli_function_none(&self, call: &CliFunctionNone) {
        match call {
            CliFunctionNone::PrintLineInteger(expr) => println!("{}", self.eval_int(expr)),
            CliFunctionNone::PrintLineFloat(expr) => println!("{}", self.eval_float(expr)),
            CliFunctionNone::PrintLineString(expr) => println!("{}", self.eval_string(expr)),
            CliFunctionNone::PrintLineBoolean(expr) => println!("{}", self.eval_boolean(expr)),
            CliFunctionNone::PrintLine => println!(),
            CliFunctionNone::PrintInteger(expr) => print!("{}", self.eval_int(expr)),
            CliFunctionNone::PrintFloat(expr) => print!("{}", self.eval_float(expr)),
            CliFunctionNone::PrintString(expr) => print!("{}", self.eval_string(expr)),
            CliFunctionNone::PrintBoolean(expr) => print!("{}", self.eval_boolean(expr)),
        }
    }

    fn cli_function_integer(&self, call: &CliFunctionInteger) -> i64 {
        match call {
            CliFunctionInteger::Prompt(expr) => prompt_int(self.eval_string(expr)),
        }
    }

    fn cli_function_string(&self, call: &CliFunctionString) -> String {
        match call {
            CliFunctionString::Prompt(expr) => prompt(self.eval_string(expr)),
        }
    }
}
