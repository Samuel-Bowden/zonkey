use self::{
    environment::{Environment, NullableReference},
    object::{NativeObject, Object},
    status::TreeWalkerStatus,
};
use crate::{
    element::*, ast::AST, err::tree_walker::TreeWalkerErr, expr::*, standard_prelude::calls::*, stmt::Stmt, event::{InterpreterEvent, PageEvent}, parser::declaration::ConstructionType,
};
use numtoa::NumToA;
use resource_loader::Address;
use std::{
    cell::RefCell,
    io::{stdout, Write},
    rc::Rc,
    sync::mpsc::Receiver,
    sync::{mpsc::Sender, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

mod environment;
mod object;
pub mod status;

pub struct TreeWalker<'a> {
    environment: Environment,
    callables: Vec<Rc<Stmt>>,
    stdout: Vec<u8>,
    interpreter_event_sender: &'a mut Sender<InterpreterEvent>,
    page_event_receiver: Receiver<PageEvent>,
    element_id: u64,
}

impl<'a> TreeWalker<'a> {
    pub fn run(
        ast: AST,
        interpreter_event_sender: &'a mut Sender<InterpreterEvent>,
        page_event_receiver: Receiver<PageEvent>,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        let mut tree_walker = Self {
            environment: Environment::new(),
            callables: ast.callable,
            stdout: vec![],
            interpreter_event_sender,
            page_event_receiver,
            element_id: 0,
        };

        let result = tree_walker.interpret(&ast.start);
        stdout().write_all(&tree_walker.stdout).unwrap();
        tree_walker.stdout.clear();
        result
    }

    fn next_element_id(&mut self) -> u64 {
        let id = self.element_id;
        self.element_id += 1;
        id
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
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
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
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
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
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
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
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
                    .borrow_mut()
                    .assign_boolean(*id, boolean, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // Class statements
            Stmt::ObjectVariableInitialisation(expr) => {
                let object = self.eval_object(expr)?;
                self.environment.push_object(NullableReference::Some(object));
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
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
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
                .get_object(*obj_id)?
                .extract_zonkey_object()
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
                .get_object(*obj_id)?
                .extract_zonkey_object()
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
                .get_object(*obj_id)?
                .extract_zonkey_object()
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
                .get_object(*obj_id)?
                .extract_zonkey_object()
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
            ObjectExpr::Variable(id) => self.environment.get_object(*id),
            ObjectExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnObject(v) => Ok(v),
                v => panic!("Call did not return correct type - {:?} was returned", v),
            },
            ObjectExpr::NativeCall(call) => self.native_call_object(call),
            ObjectExpr::Constructor(properties) => {
                let mut object = Environment::new();

                for property in properties.iter() {
                    match property {
                        ConstructionType::Integer => object.push_int(0),
                        ConstructionType::Float => object.push_float(0.),
                        ConstructionType::String => object.push_string(String::new()),
                        ConstructionType::Boolean => object.push_boolean(false),
                        ConstructionType::NullPointer(prop_name) => object.push_object(NullableReference::None(prop_name.clone())),
                    }
                }

                Ok(Object::Zonkey(Rc::new(RefCell::new(object))))
            }
            ObjectExpr::Property(obj_id, id) => Ok(self
                .environment
                .get_object(*obj_id)?
                .extract_zonkey_object()
                .borrow_mut()
                .get_object(*id)?),
        }
    }

    fn native_call_none(&mut self, call: &NativeCallNone) -> Result<(), TreeWalkerErr> {
        match call {
            NativeCallNone::Print(expr, line) => match &**expr {
                Expr::Integer(expr) => {
                    let mut buffer = [0u8; 20];
                    let int = self.eval_int(expr)?.numtoa(10, &mut buffer);
                    self.stdout.extend_from_slice(int);
                    if *line {
                        self.stdout.extend_from_slice(b"\n");
                    }
                }
                Expr::Float(expr) => {
                    let mut buffer = ryu::Buffer::new();
                    let float = buffer.format(self.eval_float(expr)?).as_bytes();
                    self.stdout.extend_from_slice(float);
                    if *line {
                        self.stdout.extend_from_slice(b"\n");
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

            NativeCallNone::Sleep(duration) => {
                let duration = self.eval_int(duration)?;
                sleep(Duration::from_millis(duration as u64));
                stdout().write_all(&self.stdout.as_slice()).ok();
                stdout().flush().ok();
                self.stdout.clear();
                self.interpreter_event_sender.send(InterpreterEvent::Update).ok();
            }

            NativeCallNone::SetPage(page) => {
                let mut page = self.eval_object(page)?;

                self.interpreter_event_sender
                    .send(InterpreterEvent::SetPage(Arc::clone(page.extract_native_object().extract_page())))
                    .ok();
            }

            NativeCallNone::CloseTab => {
                self.interpreter_event_sender
                    .send(InterpreterEvent::CloseTab)
                    .ok();
                return Err(TreeWalkerErr::Exit);
            }

            NativeCallNone::WriteString(location, string) => {
                let location = self.eval_string(location)?;
                let string = self.eval_string(string)?;

                match Address::new(&location).write_string(string) {
                    Ok(string) => string,
                    Err(e) => return Err(TreeWalkerErr::WriteAddressFailed(e.to_string())),
                };
            }

            NativeCallNone::OpenLink(link) => {
                let link = self.eval_string(&link)?;
                self.interpreter_event_sender
                    .send(InterpreterEvent::OpenLink(link))
                    .ok();
            }
        }

        Ok(())
    }

    fn native_call_string(&mut self, call: &NativeCallString) -> Result<String, TreeWalkerErr> {
        match call {
            NativeCallString::Prompt(expr) => {
                let prompt = self.eval_string(expr)?;

                self.stdout.extend_from_slice(prompt.as_bytes());
                self.stdout.extend_from_slice(" ".as_bytes());
                stdout().write_all(&self.stdout.as_slice()).unwrap();
                stdout().flush().unwrap();
                self.stdout.clear();

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                Ok(input.trim().to_string())
            }
            NativeCallString::GetInputText(input) => {
                let mut input = self.eval_object(input)?;

                let text = input
                    .extract_native_object()
                    .extract_input()
                    .lock()
                    .unwrap()
                    .text
                    .clone();

                Ok(text)
            }

            NativeCallString::ReadString(location) => {
                let location = self.eval_string(location)?;

                let string = match Address::new(&location).read_string() {
                    Ok(string) => string,
                    Err(e) => return Err(TreeWalkerErr::ReadAddressFailed(e.to_string())),
                };

                Ok(string)
            }
        }
    }

    fn native_call_boolean(&mut self, call: &NativeCallBoolean) -> Result<bool, TreeWalkerErr> {
        match call {
            NativeCallBoolean::WaitForEvent => {
                self.interpreter_event_sender
                    .send(InterpreterEvent::Update)
                    .ok();
                match self.page_event_receiver.recv() {
                    Ok(PageEvent::ButtonPress(button)) => {
                        button.lock().unwrap().clicked = true;
                        Ok(true)
                    }
                    Ok(PageEvent::InputConfirmed(input)) => {
                        input.lock().unwrap().confirmed = true;
                        Ok(true)
                    }
                    Err(_) => Ok(false),
                }
            }

            NativeCallBoolean::ButtonClicked(object) => {
                let mut object = self.eval_object(object)?;

                let mut button = object
                    .extract_native_object()
                    .extract_button()
                    .lock()
                    .unwrap();

                if button.clicked {
                    button.clicked = false;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }

            NativeCallBoolean::InputConfirmed(object) => {
                let mut object = self.eval_object(object)?;

                let mut input = object
                    .extract_native_object()
                    .extract_input()
                    .lock()
                    .unwrap();

                if input.confirmed {
                    input.confirmed = false;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    fn native_obj_to_element(obj: &NativeObject) -> ElementType {
        match obj {
            NativeObject::Page(_) => unreachable!("Cannot add page to a page"),
            NativeObject::Button(button) => ElementType::Button(Arc::clone(button)),
            NativeObject::Text(text) => ElementType::Text(Arc::clone(text)),
            NativeObject::Hyperlink(hyperlink) => ElementType::Hyperlink(Arc::clone(hyperlink)),
            NativeObject::Input(input) => ElementType::Input(Arc::clone(input)),
            NativeObject::Row(row) => ElementType::Row(Arc::clone(row)),
            NativeObject::Column(column) => ElementType::Column(Arc::clone(column)),
            NativeObject::Image(image) => ElementType::Image(Arc::clone(image)),
        }
    }

    fn native_call_object(&mut self, call: &NativeCallObject) -> Result<Object, TreeWalkerErr> {
        match call {
            NativeCallObject::PageAddElement(page_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut page_obj = self.eval_object(page_obj)?;

                {
                    let mut page = page_obj
                        .extract_native_object()
                        .extract_page()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    page.elements
                        .push((element.get_id(), Self::native_obj_to_element(element)));
                }

                Ok(page_obj)
            }

            NativeCallObject::PageRemoveElement(page_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut page_obj = self.eval_object(page_obj)?;

                {
                    let mut page = page_obj
                        .extract_native_object()
                        .extract_page()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    let pos = page
                        .elements
                        .iter()
                        .position(|&(id, _)| id == element.get_id());

                    if let Some(pos) = pos {
                        page.elements.remove(pos);
                    }
                }

                Ok(page_obj)
            }

            NativeCallObject::RowAddElement(row_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut row_obj = self.eval_object(row_obj)?;

                {
                    let mut row = row_obj
                        .extract_native_object()
                        .extract_row()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    row.elements
                        .push((element.get_id(), Self::native_obj_to_element(element)));
                }

                Ok(row_obj)
            }

            NativeCallObject::RowRemoveElement(row_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut row_obj = self.eval_object(row_obj)?;

                {
                    let mut row = row_obj
                        .extract_native_object()
                        .extract_row()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    let pos = row 
                        .elements
                        .iter()
                        .position(|&(id, _)| id == element.get_id());

                    if let Some(pos) = pos {
                        row.elements.remove(pos);
                    }
                }

                Ok(row_obj)
            }

            NativeCallObject::ColumnAddElement(column_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut column_obj = self.eval_object(column_obj)?;

                {
                    let mut column = column_obj
                        .extract_native_object()
                        .extract_column()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    column.elements
                        .push((element.get_id(), Self::native_obj_to_element(element)));
                }

                Ok(column_obj)
            }

            NativeCallObject::ColumnRemoveElement(column_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut column_obj = self.eval_object(column_obj)?;

                {
                    let mut column = column_obj
                        .extract_native_object()
                        .extract_column()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    let pos = column 
                        .elements
                        .iter()
                        .position(|&(id, _)| id == element.get_id());

                    if let Some(pos) = pos {
                        column.elements.remove(pos);
                    }
                }

                Ok(column_obj)
            }

            NativeCallObject::PageConstructor => {
                let page = Arc::new(Mutex::new(Page {
                    id: self.next_element_id(),
                    title: "Unnamed Application".to_string(),
                    elements: vec![],
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                    center: false,
                    max_width: None,
                }));

                Ok(Object::Native(NativeObject::Page(page)))
            }

            NativeCallObject::ButtonConstructor(text) => {
                let text = self.eval_string(text)?;
                let button = Arc::new(Mutex::new(Button {
                    id: self.next_element_id(),
                    text,
                    bg_red: 0.5,
                    bg_green: 0.5,
                    bg_blue: 0.5,
                    txt_red: 1.,
                    txt_green: 1.,
                    txt_blue: 1.,
                    clicked: false,
                    vertical_padding: 10.,
                    horizontal_padding: 10.,
                    width_fill: false,
                }));
                Ok(Object::Native(NativeObject::Button(button)))
            }

            NativeCallObject::ButtonSetText(object, text) => {
                let mut object = self.eval_object(object)?;
                let text = self.eval_string(text)?;

                object
                    .extract_native_object()
                    .extract_button()
                    .lock()
                    .unwrap()
                    .text = text;

                Ok(object)
            }

            NativeCallObject::TextConstructor(value) => {
                let value = self.eval_string(value)?;
                let text = Arc::new(Mutex::new(Text {
                    id: self.next_element_id(),
                    size: 20.,
                    value,
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                }));
                Ok(Object::Native(NativeObject::Text(text)))
            }

            NativeCallObject::TextSetValue(text, value) => {
                let mut object = self.eval_object(text)?;
                let value = self.eval_string(value)?;

                object
                    .extract_native_object()
                    .extract_text()
                    .lock()
                    .unwrap()
                    .value = value;

                Ok(object)
            }

            NativeCallObject::HyperlinkConstructor(text, link) => {
                let text = self.eval_string(text)?;
                let link = self.eval_string(link)?;
                let hyperlink = Arc::new(Mutex::new(Hyperlink {
                    id: self.next_element_id(),
                    link,
                    text,
                }));
                Ok(Object::Native(NativeObject::Hyperlink(hyperlink)))
            }

            NativeCallObject::InputConstructor(placeholder) => {
                let placeholder = self.eval_string(placeholder)?;
                let input = Arc::new(Mutex::new(Input {
                    id: self.next_element_id(),
                    placeholder,
                    text: String::new(),
                    confirmed: false,
                }));
                Ok(Object::Native(NativeObject::Input(input)))
            }

            NativeCallObject::ImageConstructor(link) => {
                let link = self.eval_string(link)?;
                let image = Arc::new(Mutex::new(Image {
                    data: None,
                    id: self.next_element_id(),
                    max_width: None,
                }));

                let image_ref = Arc::clone(&image);

                let sender_clone = self.interpreter_event_sender.clone();

                thread::spawn(move || {
                    let data = Address::new(&link).load_image();
                    image_ref.lock().unwrap().data = Some(data);
                    sender_clone.send(InterpreterEvent::Update).unwrap();
                });

                Ok(Object::Native(NativeObject::Image(image)))
            }

            NativeCallObject::TextSetSize(object, size) => {
                let mut object = self.eval_object(object)?;
                let size = self.eval_float(size)?;

                object
                    .extract_native_object()
                    .extract_text()
                    .lock()
                    .unwrap()
                    .size = size as f32;

                Ok(object)
            }

            NativeCallObject::TextSetColour(object, red, green, blue) => {
                let mut object = self.eval_object(object)?;
                let red = self.eval_float(red)?;
                let green = self.eval_float(green)?;
                let blue = self.eval_float(blue)?;

                {
                    let mut text = object
                        .extract_native_object()
                        .extract_text()
                        .lock()
                        .unwrap();

                    text.red = red as f32;
                    text.green = green as f32;
                    text.blue = blue as f32;
                }

                Ok(object)
            }

            NativeCallObject::ButtonSetBackgroundColour(object, red, green, blue) => {
                let mut object = self.eval_object(object)?;
                let red = self.eval_float(red)?;
                let green = self.eval_float(green)?;
                let blue = self.eval_float(blue)?;

                {
                    let mut button = object
                        .extract_native_object()
                        .extract_button()
                        .lock()
                        .unwrap();

                    button.bg_red = red as f32;
                    button.bg_green = green as f32;
                    button.bg_blue = blue as f32;
                }

                Ok(object)
            }

            NativeCallObject::ButtonSetTextColour(object, red, green, blue) => {
                let mut object = self.eval_object(object)?;
                let red = self.eval_float(red)?;
                let green = self.eval_float(green)?;
                let blue = self.eval_float(blue)?;

                {
                    let mut button = object
                        .extract_native_object()
                        .extract_button()
                        .lock()
                        .unwrap();

                    button.txt_red = red as f32;
                    button.txt_green = green as f32;
                    button.txt_blue = blue as f32;
                }

                Ok(object)
            }

            NativeCallObject::ButtonSetPadding(object, vertical, horizontal) => {
                let mut object = self.eval_object(object)?;
                let vertical = self.eval_float(vertical)?;
                let horizontal = self.eval_float(horizontal)?;

                {
                    let mut button = object
                        .extract_native_object()
                        .extract_button()
                        .lock()
                        .unwrap();

                    button.vertical_padding = vertical as f32;
                    button.horizontal_padding = horizontal as f32;
                }

                Ok(object)
            }

            NativeCallObject::PageSetTitle(page, title) => {
                let mut object = self.eval_object(page)?;
                let title = self.eval_string(title)?;

                object
                    .extract_native_object()
                    .extract_page()
                    .lock()
                    .unwrap()
                    .title = title;

                Ok(object)
            }

            NativeCallObject::PageSetBackgroundColour(page, red, green, blue) => {
                let mut object = self.eval_object(page)?;
                let red = self.eval_float(red)?;
                let green = self.eval_float(green)?;
                let blue = self.eval_float(blue)?;

                {
                    let mut page = object
                        .extract_native_object()
                        .extract_page()
                        .lock()
                        .unwrap();

                    page.red = red as f32;
                    page.green = green as f32;
                    page.blue = blue as f32;
                }

                Ok(object)
            }

            NativeCallObject::RowConstructor => {
                let row = Arc::new(Mutex::new(Row {
                    id: self.next_element_id(),
                    elements: vec![],
                    center: false,
                }));
                Ok(Object::Native(NativeObject::Row(row)))
            }

            NativeCallObject::ColumnConstructor => {
                let column = Arc::new(Mutex::new(Column {
                    id: self.next_element_id(),
                    elements: vec![],
                    max_width: None,
                }));
                Ok(Object::Native(NativeObject::Column(column)))
            }

            NativeCallObject::ButtonSetWidthFill(obj) => {
                let mut button = self.eval_object(obj)?;

                button
                    .extract_native_object()
                    .extract_button()
                    .lock()
                    .unwrap()
                    .width_fill = true;

                Ok(button)
            }

            NativeCallObject::ColumnSetMaxWidth(obj, width) => {
                let mut button = self.eval_object(obj)?;
                let width = self.eval_float(width)?;

                button
                    .extract_native_object()
                    .extract_column()
                    .lock()
                    .unwrap()
                    .max_width = Some(width as f32);

                Ok(button)
            }

            NativeCallObject::RowCenter(obj) => {
                let mut row = self.eval_object(obj)?;

                row.extract_native_object()
                    .extract_row()
                    .lock()
                    .unwrap()
                    .center = true;

                Ok(row)
            }

            NativeCallObject::ImageSetMaxWidth(obj, width) => {
                let mut image = self.eval_object(obj)?;
                let width = self.eval_float(width)?;

                image
                    .extract_native_object()
                    .extract_image()
                    .lock()
                    .unwrap()
                    .max_width = Some(width as f32);

                Ok(image)
            }

            NativeCallObject::PageCenter(page) => {
                let mut page = self.eval_object(page)?;

                page 
                    .extract_native_object()
                    .extract_page()
                    .lock()
                    .unwrap()
                    .center = true;

                Ok(page)
            }

            NativeCallObject::PageSetMaxWidth(page, max_width) => {
                let mut page = self.eval_object(page)?;
                let max_width = self.eval_float(max_width)?;

                page 
                    .extract_native_object()
                    .extract_page()
                    .lock()
                    .unwrap()
                    .max_width = Some(max_width as f32);

                Ok(page)
            }

            NativeCallObject::InputSetText(input, text) => {
                let mut input = self.eval_object(input)?;
                let text = self.eval_string(text)?;

                input 
                    .extract_native_object()
                    .extract_input()
                    .lock()
                    .unwrap()
                    .text = text;

                Ok(input)
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
                    environment.push_object(NullableReference::Some(self.eval_object(expr)?));
                }
            }
        }

        std::mem::swap(&mut environment, &mut self.environment);

        let callable = &self.callables[id];

        let result = self.interpret(&Rc::clone(callable));

        std::mem::swap(&mut environment, &mut self.environment);

        result
    }
}
