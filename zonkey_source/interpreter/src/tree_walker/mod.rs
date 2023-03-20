use numtoa::NumToA;
use rustc_hash::FxHashMap;

use self::{environment::Environment, status::TreeWalkerStatus};
use crate::{
    ast::AST,
    err::tree_walker::TreeWalkerErr,
    event::{BrowserEvent, Button, InterpreterEvent, Text},
    expr::*,
    prelude::calls::*,
    stmt::Stmt,
};
use std::{
    cell::RefCell,
    io::{stdout, BufWriter, StdoutLock, Write},
    rc::Rc,
    sync::mpsc::Receiver,
    sync::mpsc::Sender,
};

mod environment;
pub mod status;

type Object = Rc<RefCell<Environment>>;

pub struct TreeWalker<'a> {
    environment: Environment,
    callables: Vec<Rc<Stmt>>,
    stdout: BufWriter<StdoutLock<'a>>,
    sender: Sender<InterpreterEvent>,
    receiver: Receiver<BrowserEvent>,
    ui_elements: FxHashMap<i64, Object>,
    next_id_ui: i64,
}

impl<'a> TreeWalker<'a> {
    pub fn run(
        ast: AST,
        sender: Sender<InterpreterEvent>,
        receiver: Receiver<BrowserEvent>,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        let mut tree_walker = Self {
            environment: Environment::new(),
            callables: ast.callable,
            stdout: BufWriter::new(stdout().lock()),
            sender,
            receiver,
            ui_elements: FxHashMap::default(),
            next_id_ui: 0,
        };

        tree_walker.interpret(&ast.start)
    }

    pub fn interpret(&mut self, statement: &Stmt) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        match statement {
            // Integer statements
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
            Stmt::IntegerPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let int = self.eval_int(expr)?;
                self.environment
                    .get_object(*obj_id)
                    .borrow_mut()
                    .assign_int(*id, int, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // Float statements
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
            Stmt::FloatPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let float = self.eval_float(expr)?;
                self.environment
                    .get_object(*obj_id)
                    .borrow_mut()
                    .assign_float(*id, float, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // String statements
            Stmt::StringVariableInitialisation(expr) => {
                let string = self.eval_string(expr)?;
                self.environment.push_string(string);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::StringVariableAssignment(id, expr, assignment_operator) => {
                let string = self.eval_string(expr)?;
                self.environment
                    .assign_string(*id, string, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::StringPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let string = self.eval_string(expr)?;
                self.environment
                    .get_object(*obj_id)
                    .borrow_mut()
                    .assign_string(*id, string, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // Boolean statements
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
            Stmt::BooleanPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let boolean = self.eval_boolean(expr)?;
                self.environment
                    .get_object(*obj_id)
                    .borrow_mut()
                    .assign_boolean(*id, boolean, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // Class statements
            Stmt::ObjectVariableInitialisation(expr) => {
                let object = self.eval_object(expr)?;
                self.environment.push_object(object);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::ObjectVariableAssignment(id, expr, assignment_operator) => {
                let object = self.eval_object(expr)?;
                self.environment
                    .assign_object(*id, object, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::ObjectPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let object = self.eval_object(expr)?;
                self.environment
                    .get_object(*obj_id)
                    .borrow_mut()
                    .assign_object(*id, object, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // Other statements
            Stmt::Block(statements, stack) => {
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

                self.environment.pop_stack(stack);

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
                    Expr::Object(_, expr) => {
                        self.eval_object(expr)?;
                    }
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
                Some(Expr::Object(_, expr)) => {
                    Ok(TreeWalkerStatus::ReturnObject(self.eval_object(expr)?))
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
            IntegerExpr::Property(obj_id, id) => Ok(self
                .environment
                .get_object(*obj_id)
                .borrow_mut()
                .get_int(*id)),
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
            FloatExpr::Property(obj_id, id) => Ok(self
                .environment
                .get_object(*obj_id)
                .borrow_mut()
                .get_float(*id)),
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
            StringExpr::Property(obj_id, id) => Ok(self
                .environment
                .get_object(*obj_id)
                .borrow_mut()
                .get_string(*id)),
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
            BooleanExpr::NativeCall(call) => self.native_call_boolean(call),
            BooleanExpr::Property(obj_id, id) => Ok(self
                .environment
                .get_object(*obj_id)
                .borrow_mut()
                .get_boolean(*id)),
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

    fn eval_object(&mut self, expression: &ObjectExpr) -> Result<Object, TreeWalkerErr> {
        match expression {
            ObjectExpr::Variable(id) => Ok(self.environment.get_object(*id)),
            ObjectExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnObject(v) => Ok(v),
                v => panic!("Call did not return correct type - {:?} was returned", v),
            },
            ObjectExpr::NativeCall(call) => self.native_call_object(call),
            ObjectExpr::Constructor(properties) => {
                let mut object = Environment::new();

                for property in properties {
                    match property {
                        Expr::Integer(expr) => object.push_int(self.eval_int(expr)?),
                        Expr::Float(expr) => object.push_float(self.eval_float(expr)?),
                        Expr::String(expr) => object.push_string(self.eval_string(expr)?),
                        Expr::Boolean(expr) => object.push_boolean(self.eval_boolean(expr)?),
                        Expr::Object(_, expr) => object.push_object(self.eval_object(expr)?),
                        _ => panic!("Unsupported type"),
                    }
                }

                Ok(Rc::new(RefCell::new(object)))
            }
            ObjectExpr::Property(obj_id, id) => Ok(self
                .environment
                .get_object(*obj_id)
                .borrow_mut()
                .get_object(*id)),
        }
    }

    fn native_call_none(&mut self, call: &NativeCallNone) -> Result<(), TreeWalkerErr> {
        match call {
            NativeCallNone::Print(expr, line) => match &**expr {
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

            NativeCallNone::AddButton(_, element) => {
                let element = self.eval_object(element)?;
                let text = element.borrow_mut().get_string(0);
                let red = element.borrow().get_float(0) as f32;
                let green = element.borrow().get_float(1) as f32;
                let blue = element.borrow().get_float(2) as f32;
                let id = self.next_id_ui;
                self.next_id_ui += 1;
                element
                    .borrow_mut()
                    .assign_int(0, id, &NumericAssignmentOperator::Equal);
                self.ui_elements.insert(id, Rc::clone(&element));
                self.event(InterpreterEvent::AddButton(Button {
                    id,
                    text,
                    red,
                    green,
                    blue,
                }))?;
            }

            NativeCallNone::AddText(_, element) => {
                let element = self.eval_object(element)?;
                let text = element.borrow().get_string(0);
                let size = element.borrow().get_float(0) as f32;
                let red = element.borrow().get_float(1) as f32;
                let green = element.borrow().get_float(2) as f32;
                let blue = element.borrow().get_float(3) as f32;
                let id = self.next_id_ui;
                self.next_id_ui += 1;
                element
                    .borrow_mut()
                    .assign_int(0, id, &NumericAssignmentOperator::Equal);
                self.ui_elements.insert(id, Rc::clone(&element));
                self.event(InterpreterEvent::AddText(Text {
                    id,
                    size,
                    value: text,
                    red,
                    green,
                    blue,
                }))?;
            }

            NativeCallNone::AddHyperlink(_, element) => {
                let element = self.eval_object(element)?;
                let text = element.borrow_mut().get_string(0);
                let link = element.borrow_mut().get_string(1);
                let id = self.next_id_ui;
                self.next_id_ui += 1;
                element
                    .borrow_mut()
                    .assign_int(0, id, &NumericAssignmentOperator::Equal);
                self.ui_elements.insert(id, Rc::clone(&element));
                self.event(InterpreterEvent::AddHyperlink(text, link, id))?;
            }

            NativeCallNone::AddInput(_, element) => {
                let element = self.eval_object(element)?;
                let placeholder = element.borrow_mut().get_string(0);
                let id = self.next_id_ui;
                self.next_id_ui += 1;
                element
                    .borrow_mut()
                    .assign_int(0, id, &NumericAssignmentOperator::Equal);
                self.ui_elements.insert(id, Rc::clone(&element));
                self.event(InterpreterEvent::AddInput(placeholder, id))?;
            }
        }

        Ok(())
    }

    fn native_call_string(&mut self, call: &NativeCallString) -> Result<String, TreeWalkerErr> {
        match call {
            NativeCallString::Prompt(expr) => {
                let prompt = self.eval_string(expr)?;

                self.stdout.flush().unwrap();

                write!(self.stdout, "{prompt} ").unwrap();

                self.stdout.flush().unwrap();

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                Ok(input.trim().to_string())
            }
            NativeCallString::GetInputText(input) => {
                let input = self.eval_object(input)?;
                let text = input.borrow_mut().get_string(1);
                Ok(text)
            }
        }
    }

    fn native_call_boolean(&mut self, call: &NativeCallBoolean) -> Result<bool, TreeWalkerErr> {
        match call {
            NativeCallBoolean::WaitForEvent => match self.receiver.recv() {
                Ok(BrowserEvent::ButtonPress(id)) => {
                    self.ui_elements
                        .get(&id)
                        .unwrap()
                        .borrow_mut()
                        .assign_boolean(0, true, &BooleanAssignmentOperator::Equal);
                    Ok(false)
                }
                Ok(BrowserEvent::InputConfirmed(value, id)) => {
                    self.ui_elements
                        .get(&id)
                        .unwrap()
                        .borrow_mut()
                        .assign_boolean(0, true, &BooleanAssignmentOperator::Equal);
                    self.ui_elements
                        .get(&id)
                        .unwrap()
                        .borrow_mut()
                        .assign_string(1, value, &StringAssignmentOperator::Equal);
                    Ok(false)
                }
                Err(_) => Ok(true),
            },

            NativeCallBoolean::ButtonClicked(button) => {
                let button = self.eval_object(button)?;
                let clicked = button.borrow().get_boolean(0);
                if clicked {
                    button
                        .borrow_mut()
                        .assign_boolean(0, false, &BooleanAssignmentOperator::Equal);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }

            NativeCallBoolean::InputConfirmed(input) => {
                let input = self.eval_object(input)?;
                let clicked = input.borrow().get_boolean(0);
                if clicked {
                    input
                        .borrow_mut()
                        .assign_boolean(0, false, &BooleanAssignmentOperator::Equal);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    fn native_call_object(&mut self, call: &NativeCallObject) -> Result<Object, TreeWalkerErr> {
        match call {
            NativeCallObject::ButtonConstructor(text) => {
                let mut button = Environment::new();
                let text = self.eval_string(text)?;
                button.push_string(text);
                button.push_int(0);
                button.push_boolean(false);
                button.push_float(0.5);
                button.push_float(0.5);
                button.push_float(0.5);
                Ok(Rc::new(RefCell::new(button)))
            }

            NativeCallObject::SetButtonText(object, text) => {
                let object = self.eval_object(object)?;
                let text = self.eval_string(text)?;

                object.borrow_mut().assign_string(
                    0,
                    text.to_string(),
                    &StringAssignmentOperator::Equal,
                );

                let id = object.borrow_mut().get_int(0);

                if id != 0 {
                    self.event(InterpreterEvent::SetButtonText(text, id))?;
                }

                Ok(object)
            }

            NativeCallObject::PageConstructor => {
                let button = Environment::new();
                let object = Rc::new(RefCell::new(button));
                Ok(object)
            }

            NativeCallObject::TextConstructor(text) => {
                let mut heading = Environment::new();
                let text = self.eval_string(text)?;
                heading.push_string(text);
                heading.push_int(0);
                heading.push_float(20.);
                heading.push_float(0.5);
                heading.push_float(0.5);
                heading.push_float(0.5);
                Ok(Rc::new(RefCell::new(heading)))
            }

            NativeCallObject::SetTextValue(object, text) => {
                let object = self.eval_object(object)?;
                let text = self.eval_string(text)?;

                object.borrow_mut().assign_string(
                    0,
                    text.to_string(),
                    &StringAssignmentOperator::Equal,
                );

                let id = object.borrow_mut().get_int(0);

                if id != 0 {
                    self.event(InterpreterEvent::SetTextValue(text, id))?;
                }

                Ok(object)
            }

            NativeCallObject::HyperlinkConstructor(text, link) => {
                let mut hyperlink = Environment::new();
                let text = self.eval_string(text)?;
                let link = self.eval_string(link)?;
                hyperlink.push_string(text);
                hyperlink.push_string(link);
                hyperlink.push_int(0);
                Ok(Rc::new(RefCell::new(hyperlink)))
            }

            NativeCallObject::InputConstructor(placeholder) => {
                let mut input = Environment::new();
                let placeholder = self.eval_string(placeholder)?;
                input.push_string(placeholder);
                input.push_string("".to_string());
                input.push_int(0);
                input.push_boolean(false);
                Ok(Rc::new(RefCell::new(input)))
            }

            NativeCallObject::SetTextSize(object, size) => {
                let object = self.eval_object(object)?;
                let size = self.eval_float(size)?;

                object
                    .borrow_mut()
                    .assign_float(0, size, &NumericAssignmentOperator::Equal);

                let id = object.borrow_mut().get_int(0);

                if id != 0 {
                    self.event(InterpreterEvent::SetTextSize(size as f32, id))?;
                }

                Ok(object)
            }

            NativeCallObject::SetTextColour(object, red, green, blue) => {
                let object = self.eval_object(object)?;
                let red = self.eval_float(red)?;
                let green = self.eval_float(green)?;
                let blue = self.eval_float(blue)?;
                object
                    .borrow_mut()
                    .assign_float(1, red, &NumericAssignmentOperator::Equal);
                object
                    .borrow_mut()
                    .assign_float(2, green, &NumericAssignmentOperator::Equal);
                object
                    .borrow_mut()
                    .assign_float(3, blue, &NumericAssignmentOperator::Equal);
                let id = object.borrow_mut().get_int(0);
                if id != 0 {
                    self.event(InterpreterEvent::SetTextColour(
                        red as f32,
                        green as f32,
                        blue as f32,
                        id,
                    ))?;
                }
                Ok(object)
            }

            NativeCallObject::SetButtonBackgroundColour(object, red, green, blue) => {
                let object = self.eval_object(object)?;
                let red = self.eval_float(red)?;
                let green = self.eval_float(green)?;
                let blue = self.eval_float(blue)?;
                object
                    .borrow_mut()
                    .assign_float(0, red, &NumericAssignmentOperator::Equal);
                object
                    .borrow_mut()
                    .assign_float(1, green, &NumericAssignmentOperator::Equal);
                object
                    .borrow_mut()
                    .assign_float(2, blue, &NumericAssignmentOperator::Equal);
                let id = object.borrow_mut().get_int(0);
                if id != 0 {
                    self.event(InterpreterEvent::SetButtonBackgroundColour(
                        red as f32,
                        green as f32,
                        blue as f32,
                        id,
                    ))?;
                }
                Ok(object)
            }
        }
    }

    fn eval_call(
        &mut self,
        id: usize,
        expressions: &Vec<Expr>,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        let mut environment = Environment::new();

        for expr in expressions {
            match expr {
                Expr::Integer(expr) => {
                    let integer = self.eval_int(expr)?;
                    environment.push_int(integer)
                }
                Expr::Float(expr) => {
                    let float = self.eval_float(expr)?;
                    environment.push_float(float)
                }
                Expr::String(expr) => {
                    let string = self.eval_string(expr)?;
                    environment.push_string(string)
                }
                Expr::Boolean(expr) => {
                    let boolean = self.eval_boolean(expr)?;
                    environment.push_boolean(boolean)
                }
                Expr::None(_) => panic!("Cannot pass none to a callable"),
                Expr::Object(_, expr) => {
                    environment.push_object(self.eval_object(expr)?);
                }
            }
        }

        std::mem::swap(&mut environment, &mut self.environment);

        let callable = &self.callables[id];

        let result = self.interpret(&Rc::clone(callable));

        std::mem::swap(&mut environment, &mut self.environment);

        result
    }

    fn event(&mut self, event: InterpreterEvent) -> Result<(), TreeWalkerErr> {
        match self.sender.send(event) {
            Ok(()) => Ok(()),
            Err(_) => Err(TreeWalkerErr::FailedToSendEventToBrowser),
        }
    }
}
