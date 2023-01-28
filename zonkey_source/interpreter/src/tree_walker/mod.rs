use self::status::TreeWalkerStatus;
use crate::{
    comparison::{BooleanComparision, NumericComparision, StringComparision},
    environment::Environment,
    expr::{BooleanExpr, Expr, FloatExpr, IntegerExpr, NoneExpr, StringExpr},
    function::Function,
    native_function::{
        cli_api::{
            CliFunctionInteger, CliFunctionNone, CliFunctionString,
        },
        NativeFunctionInteger, NativeFunctionNone, NativeFunctionString,
    },
    operator::{NumericOperator, StringOperator},
    stmt::Stmt,
};
use numtoa::NumToA;
use termcolor::{StandardStream, ColorSpec, Color, WriteColor};
use std::io::{stdout, BufWriter, StdoutLock, Write};

pub mod status;

pub struct TreeWalker<'a> {
    environment: Environment,
    functions: &'a Vec<Function>,
    stdout: BufWriter<StdoutLock<'a>>,
}

impl<'a> TreeWalker<'a> {
    pub fn new(functions: &'a Vec<Function>, environment: Environment) -> Self {
        Self {
            environment,
            functions,
            stdout: BufWriter::new(stdout().lock()),
        }
    }

    pub fn interpret(&mut self, statement: &Stmt) -> TreeWalkerStatus {
        match statement {
            Stmt::IntegerVariableDeclaration(expr) => {
                let int = self.eval_int(expr);
                self.environment.push_int(int);
                TreeWalkerStatus::Ok
            }
            Stmt::IntegerVariableAssignment(id, expr, assignment_operator) => {
                let int = self.eval_int(expr);
                self.environment
                    .assign_int(*id, int, assignment_operator);
                TreeWalkerStatus::Ok
            }
            Stmt::FloatVariableDeclaration(expr) => {
                let float = self.eval_float(expr);
                self.environment.push_float(float);
                TreeWalkerStatus::Ok
            }
            Stmt::FloatVariableAssignment(id, expr, assignment_operator) => {
                let float = self.eval_float(expr);
                self.environment
                    .assign_float(*id, float, assignment_operator);
                TreeWalkerStatus::Ok
            }
            Stmt::StringVariableDeclaration(expr) => {
                let string = self.eval_string(expr);
                self.environment.push_string(string);
                TreeWalkerStatus::Ok
            }
            Stmt::StringVariableAssignment(id, expr, assignment_operator) => {
                let string = self.eval_string(expr);
                self.environment
                    .assign_string(*id, string, assignment_operator);
                TreeWalkerStatus::Ok
            }
            Stmt::BooleanVariableDeclaration(expr) => {
                let boolean = self.eval_boolean(expr);
                self.environment.push_boolean(boolean);
                TreeWalkerStatus::Ok
            }
            Stmt::BooleanVariableAssignment(id, expr, assignment_operator) => {
                let boolean = self.eval_boolean(expr);
                self.environment
                    .assign_boolean(*id, boolean, assignment_operator);
                TreeWalkerStatus::Ok
            }
            Stmt::Block(statements, block_start_points) => {
                self.environment.push_stack();

                let mut return_value = TreeWalkerStatus::Ok;

                for statement in statements {
                    match self.interpret(statement) {
                        TreeWalkerStatus::Ok => (),
                        TreeWalkerStatus::Continue => {
                            return_value = TreeWalkerStatus::Continue;
                            break;
                        }
                        TreeWalkerStatus::Break => {
                            return_value = TreeWalkerStatus::Break;
                            break;
                        }
                        TreeWalkerStatus::ReturnInt(v) => {
                            return_value = TreeWalkerStatus::ReturnInt(v);
                            break;
                        }
                        TreeWalkerStatus::ReturnFloat(v) => {
                            return_value = TreeWalkerStatus::ReturnFloat(v);
                            break;
                        }
                        TreeWalkerStatus::ReturnString(v) => {
                            return_value = TreeWalkerStatus::ReturnString(v);
                            break;
                        }
                        TreeWalkerStatus::ReturnBoolean(v) => {
                            return_value = TreeWalkerStatus::ReturnBoolean(v);
                            break;
                        }
                        TreeWalkerStatus::ReturnNone => {
                            return_value = TreeWalkerStatus::ReturnNone;
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
                    TreeWalkerStatus::Ok
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

                TreeWalkerStatus::Ok
            }
            Stmt::While(condition, block) => {
                while self.eval_boolean(condition) {
                    match self.interpret(block) {
                        TreeWalkerStatus::Ok => (),
                        TreeWalkerStatus::Continue => (),
                        TreeWalkerStatus::Break => break,
                        TreeWalkerStatus::ReturnInt(v) => {
                            return TreeWalkerStatus::ReturnInt(v)
                        }
                        TreeWalkerStatus::ReturnFloat(v) => {
                            return TreeWalkerStatus::ReturnFloat(v)
                        }
                        TreeWalkerStatus::ReturnString(v) => {
                            return TreeWalkerStatus::ReturnString(v)
                        }
                        TreeWalkerStatus::ReturnBoolean(v) => {
                            return TreeWalkerStatus::ReturnBoolean(v)
                        }
                        TreeWalkerStatus::ReturnNone => return TreeWalkerStatus::ReturnNone,
                    }
                }

                TreeWalkerStatus::Ok
            }
            Stmt::Loop(block) => {
                loop {
                    match self.interpret(block) {
                        TreeWalkerStatus::Ok => (),
                        TreeWalkerStatus::Continue => (),
                        TreeWalkerStatus::Break => break,
                        TreeWalkerStatus::ReturnInt(v) => {
                            return TreeWalkerStatus::ReturnInt(v)
                        }
                        TreeWalkerStatus::ReturnFloat(v) => {
                            return TreeWalkerStatus::ReturnFloat(v)
                        }
                        TreeWalkerStatus::ReturnString(v) => {
                            return TreeWalkerStatus::ReturnString(v)
                        }
                        TreeWalkerStatus::ReturnBoolean(v) => {
                            return TreeWalkerStatus::ReturnBoolean(v)
                        }
                        TreeWalkerStatus::ReturnNone => return TreeWalkerStatus::ReturnNone,
                    }
                }

                TreeWalkerStatus::Ok
            }
            Stmt::Break => TreeWalkerStatus::Break,
            Stmt::Continue => TreeWalkerStatus::Continue,
            Stmt::Return(expr) => match expr {
                Some(Expr::Integer(expr)) => TreeWalkerStatus::ReturnInt(self.eval_int(expr)),
                Some(Expr::Float(expr)) => TreeWalkerStatus::ReturnFloat(self.eval_float(expr)),
                Some(Expr::String(expr)) => {
                    TreeWalkerStatus::ReturnString(self.eval_string(expr))
                }
                Some(Expr::Boolean(expr)) => {
                    TreeWalkerStatus::ReturnBoolean(self.eval_boolean(expr))
                }
                Some(Expr::None(expr)) => {
                    self.eval_none(expr);
                    TreeWalkerStatus::ReturnNone
                }
                None => TreeWalkerStatus::ReturnNone,
            },
        }
    }

    fn eval_int(&mut self, expression: &IntegerExpr) -> i64 {
        match expression {
            IntegerExpr::Binary {
                left,
                operator,
                right,
            } => match operator {
                NumericOperator::Add => self.eval_int(left) + self.eval_int(right),
                NumericOperator::Subtract => self.eval_int(left) - self.eval_int(right),
                NumericOperator::Multiply => self.eval_int(left) * self.eval_int(right),
                NumericOperator::Divide => {
                    let left = self.eval_int(left);
                    let right = self.eval_int(right);
                    
                    if right == 0 {
                        let mut stderr = StandardStream::stderr(termcolor::ColorChoice::Always);

                        stderr
                            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                            .unwrap();
                        write!(stderr, "(FATAL RUNTIME ERROR) ").unwrap();
                        stderr.reset().unwrap();

                        writeln!(stderr, "Attempted to divide {left} by {right}. Aborting execution.").unwrap();

                        std::process::exit(1);
                    }

                    left / right
                },
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
                    TreeWalkerStatus::ReturnInt(v) => v,
                    _ => panic!("Function did not return the correct type"),
                }
            }
            IntegerExpr::NativeCall(call) => self.native_call_integer(call),
        }
    }

    fn eval_float(&mut self, expression: &FloatExpr) -> f64 {
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
                    TreeWalkerStatus::ReturnFloat(v) => v,
                    _ => panic!("Function did not return the correct type"),
                }
            }
        }
    }

    fn eval_string(&mut self, expression: &StringExpr) -> String {
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
                    TreeWalkerStatus::ReturnString(v) => v,
                    _ => panic!("Function did not return the correct type"),
                }
            }
        }
    }

    fn eval_boolean(&mut self, expression: &BooleanExpr) -> bool {
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
                NumericComparision::MoreEqual => self.eval_float(left) >= self.eval_float(right),
                NumericComparision::LessEqual => self.eval_float(left) <= self.eval_float(right),
                NumericComparision::More => self.eval_float(left) > self.eval_float(right),
                NumericComparision::Less => self.eval_float(left) < self.eval_float(right),
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
                    TreeWalkerStatus::ReturnBoolean(v) => v,
                    _ => panic!("Function did not return the correct type"),
                }
            }
        }
    }

    fn eval_none(&mut self, expression: &NoneExpr) {
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
                    TreeWalkerStatus::ReturnNone | TreeWalkerStatus::Ok => (),
                    _ => panic!("Function did not return the correct type"),
                }
            }
        }
    }

    fn native_call_none(&mut self, call: &NativeFunctionNone) {
        match call {
            NativeFunctionNone::Cli(call) => self.cli_function_none(call),
        }
    }

    fn native_call_integer(&mut self, call: &NativeFunctionInteger) -> i64 {
        match call {
            NativeFunctionInteger::Cli(call) => self.cli_function_integer(call),
        }
    }

    fn native_call_string(&mut self, call: &NativeFunctionString) -> String {
        match call {
            NativeFunctionString::Cli(call) => self.cli_function_string(call),
        }
    }

    fn cli_function_none(&mut self, call: &CliFunctionNone) {
        match call {
            CliFunctionNone::PrintLineInteger(expr) => {
                let mut buffer = [0u8; 20];
                let int = self.eval_int(expr).numtoa(10, &mut buffer);
                self.stdout
                    .write(int)
                    .unwrap();
                self.stdout.write(b"\n").unwrap();
            }
            CliFunctionNone::PrintLineFloat(expr) => {
                let mut buffer = ryu::Buffer::new();
                let float = buffer.format(self.eval_float(expr)).as_bytes();
                self.stdout
                    .write(float)
                    .unwrap();
                self.stdout.write(b"\n").unwrap();
            }
            CliFunctionNone::PrintLineString(expr) => {
                let string = self.eval_string(expr);
                writeln!(self.stdout, "{}", string).unwrap()
            }
            CliFunctionNone::PrintLineBoolean(expr) => {
                let boolean = self.eval_boolean(expr);
                writeln!(self.stdout, "{}", boolean).unwrap()
            }
            CliFunctionNone::PrintLine => writeln!(self.stdout).unwrap(),
            CliFunctionNone::PrintInteger(expr) => {
                let mut buffer = [0u8; 20];
                let int = self.eval_int(expr).numtoa(10, &mut buffer);
                self.stdout
                    .write(int)
                    .unwrap();
            }
            CliFunctionNone::PrintFloat(expr) => {
                let mut buffer = ryu::Buffer::new();
                let float = buffer.format(self.eval_float(expr)).as_bytes();
                self.stdout
                    .write(float)
                    .unwrap();
            }
            CliFunctionNone::PrintString(expr) => {
                let string = self.eval_string(expr);
                write!(self.stdout, "{}", string).unwrap()
            }
            CliFunctionNone::PrintBoolean(expr) => {
                let boolean = self.eval_boolean(expr);
                write!(self.stdout, "{}", boolean).unwrap()
            }
        }
    }

    fn cli_function_integer(&mut self, call: &CliFunctionInteger) -> i64 {
        match call {
            CliFunctionInteger::Prompt(expr) => {
                let prompt = self.eval_string(expr);
                
                self.stdout.flush().unwrap();

                write!(self.stdout, "{prompt} ").unwrap();

                self.stdout.flush().unwrap();

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                input.trim().parse().unwrap()
            }
        }
    }

    fn cli_function_string(&mut self, call: &CliFunctionString) -> String {
        match call {
            CliFunctionString::Prompt(expr) => {
                let prompt = self.eval_string(expr);
                
                self.stdout.flush().unwrap();

                write!(self.stdout, "{prompt} ").unwrap();

                self.stdout.flush().unwrap();

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                input.trim().to_string()
            },
        }
    }
}
