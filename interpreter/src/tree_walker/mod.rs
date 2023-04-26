use self::{
    err::TreeWalkerErr,
    object::{NativeObject, Object},
    state::{NullableReference, State},
    status::TreeWalkerStatus,
};
use crate::{
    ast::AST,
    element::*,
    event::{InterpreterEvent, PageEvent},
    expr::*,
    parser::declaration::ConstructionType,
    stmt::Stmt,
    tree_walker_debug, PermissionLevel,
};
use std::{
    cell::RefCell,
    io::{stdout, Write},
    rc::Rc,
    sync::mpsc::Receiver,
    sync::{mpsc::Sender, Arc, Mutex},
};

pub mod err;
mod native_call;
mod object;
pub mod state;
pub mod status;

pub struct TreeWalker<'a> {
    state: State,
    callables: Vec<Rc<Stmt>>,
    stdout: Vec<u8>,
    interpreter_event_sender: &'a mut Sender<InterpreterEvent>,
    page_event_receiver: Receiver<PageEvent>,
    element_id: u64,
    permission_level: PermissionLevel,
    arguments: Arc<Mutex<Vec<String>>>,
}

impl<'a> TreeWalker<'a> {
    pub fn run(
        ast: AST,
        interpreter_event_sender: &'a mut Sender<InterpreterEvent>,
        page_event_receiver: Receiver<PageEvent>,
        permission_level: PermissionLevel,
        arguments: Vec<String>,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        let mut tree_walker = Self {
            state: State::new(),
            callables: ast.callable,
            stdout: vec![],
            interpreter_event_sender,
            page_event_receiver,
            element_id: 0,
            permission_level,
            arguments: Arc::new(Mutex::new(arguments)),
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
        tree_walker_debug!(format!("Interpret statement: {:?}", statement).as_str());

        match statement {
            // Integer statements
            Stmt::IntegerVariableInitialisation(expr) => {
                let int = self.eval_int(expr)?;
                self.state.push_int(int);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::IntegerVariableAssignment(id, expr, assignment_operator) => {
                let int = self.eval_int(expr)?;
                self.state.assign_int(*id, int, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::IntegerPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let int = self.eval_int(expr)?;
                self.state
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
                    .borrow_mut()
                    .assign_int(*id, int, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // Float statements
            Stmt::FloatVariableInitialisation(expr) => {
                let float = self.eval_float(expr)?;
                self.state.push_float(float);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::FloatVariableAssignment(id, expr, assignment_operator) => {
                let float = self.eval_float(expr)?;
                self.state.assign_float(*id, float, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::FloatPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let float = self.eval_float(expr)?;
                self.state
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
                    .borrow_mut()
                    .assign_float(*id, float, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // String statements
            Stmt::StringVariableInitialisation(expr) => {
                let string = self.eval_string(expr)?;
                self.state.push_string(string);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::StringVariableAssignment(id, expr, assignment_operator) => {
                let string = self.eval_string(expr)?;
                self.state.assign_string(*id, string, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::StringPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let string = self.eval_string(expr)?;
                self.state
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
                    .borrow_mut()
                    .assign_string(*id, string, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // Boolean statements
            Stmt::BooleanVariableInitialisation(expr) => {
                let boolean = self.eval_boolean(expr)?;
                self.state.push_boolean(boolean);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::BooleanVariableAssignment(id, expr, assignment_operator) => {
                let boolean = self.eval_boolean(expr)?;
                self.state.assign_boolean(*id, boolean, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::BooleanPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let boolean = self.eval_boolean(expr)?;
                self.state
                    .get_object(*obj_id)?
                    .extract_zonkey_object()
                    .borrow_mut()
                    .assign_boolean(*id, boolean, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }

            // Class statements
            Stmt::ObjectVariableInitialisation(expr) => {
                let object = self.eval_object(expr)?;
                self.state.push_object(NullableReference::Some(object));
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::SelfInitialisation(expr) => {
                let object = self.eval_object(expr)?;
                self.state.set_self(NullableReference::Some(object));
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::ObjectVariableAssignment(id, expr, assignment_operator) => {
                let object = self.eval_object(expr)?;
                self.state.assign_object(*id, object, assignment_operator);
                Ok(TreeWalkerStatus::Ok)
            }
            Stmt::ObjectPropertyAssignment(obj_id, id, expr, assignment_operator) => {
                let object = self.eval_object(expr)?;
                self.state
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

                self.state.pop_stack(stack);

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
                NumericOperator::Divide(token) => {
                    let left = self.eval_int(left)?;
                    let right = self.eval_int(right)?;

                    if right == 0 {
                        Err(TreeWalkerErr::DivisionByZero(token.clone()))
                    } else {
                        Ok(left / right)
                    }
                }
            },
            IntegerExpr::Unary(unary_operator, expr) => match unary_operator {
                NumericUnaryOperator::Minus => Ok(-self.eval_int(expr)?),
            },
            IntegerExpr::Variable(id) => Ok(self.state.get_int(*id)),
            IntegerExpr::Literal(val) => Ok(*val),
            IntegerExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnInt(v) => Ok(v),
                _ => panic!("Call did not return correct type"),
            },
            IntegerExpr::Property(obj_id, id) => Ok(self
                .state
                .get_object(*obj_id)?
                .extract_zonkey_object()
                .borrow_mut()
                .get_int(*id)),
            IntegerExpr::NativeCall(call) => self.native_call_integer(call),
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
                NumericOperator::Divide(_) => Ok(self.eval_float(left)? / self.eval_float(right)?),
            },
            FloatExpr::Unary(unary_operator, expr) => match unary_operator {
                NumericUnaryOperator::Minus => Ok(-self.eval_float(expr)?),
            },
            FloatExpr::Variable(id) => Ok(self.state.get_float(*id)),
            FloatExpr::Literal(val) => Ok(*val),
            FloatExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnFloat(v) => Ok(v),
                _ => panic!("Call did not return correct type"),
            },
            FloatExpr::NativeCall(call) => self.native_call_float(call),
            FloatExpr::Property(obj_id, id) => Ok(self
                .state
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
            StringExpr::Variable(id) => Ok(self.state.get_string(*id)),
            StringExpr::Literal(val) => Ok(val.to_string()),
            StringExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnString(v) => Ok(v),
                _ => panic!("Call did not return correct type"),
            },
            StringExpr::NativeCall(call) => self.native_call_string(call),
            StringExpr::Property(obj_id, id) => Ok(self
                .state
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
            BooleanExpr::Variable(id) => Ok(self.state.get_boolean(*id)),
            BooleanExpr::Literal(val) => Ok(*val),
            BooleanExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnBoolean(v) => Ok(v),
                _ => panic!("Call did not return correct type"),
            },
            BooleanExpr::Unary(unary_operator, expr) => match unary_operator {
                BooleanUnaryOperator::Bang => Ok(!self.eval_boolean(expr)?),
            },
            BooleanExpr::NativeCall(call) => self.native_call_boolean(call),
            BooleanExpr::Property(obj_id, id) => Ok(self
                .state
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
            ObjectExpr::Variable(id) => self.state.get_object(*id),
            ObjectExpr::Call(id, expressions) => match self.eval_call(*id, expressions)? {
                TreeWalkerStatus::ReturnObject(v) => Ok(v),
                v => panic!("Call did not return correct type - {:?} was returned", v),
            },
            ObjectExpr::NativeCall(call) => self.native_call_object(call),
            ObjectExpr::Constructor(properties) => {
                let mut object = State::new();

                for property in properties.iter() {
                    match property {
                        ConstructionType::Integer => object.push_int(0),
                        ConstructionType::Float => object.push_float(0.),
                        ConstructionType::String => object.push_string(String::new()),
                        ConstructionType::Boolean => object.push_boolean(false),
                        ConstructionType::NullPointer(prop_name) => {
                            object.push_object(NullableReference::None(prop_name.clone()))
                        }
                    }
                }

                Ok(Object::Zonkey(Rc::new(RefCell::new(object))))
            }
            ObjectExpr::Property(obj_id, id) => Ok(self
                .state
                .get_object(*obj_id)?
                .extract_zonkey_object()
                .borrow_mut()
                .get_object(*id)?),
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
            _ => unreachable!("Not applicable for this object"),
        }
    }

    fn eval_call(
        &mut self,
        id: usize,
        expressions: &Vec<Expr>,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        tree_walker_debug!(format!("Executing native callable with id {}", id).as_str());

        let mut state = State::new();

        for expr in expressions {
            match expr {
                Expr::Integer(expr) => {
                    let integer = self.eval_int(expr)?;
                    state.push_int(integer)
                }
                Expr::Float(expr) => {
                    let float = self.eval_float(expr)?;
                    state.push_float(float)
                }
                Expr::String(expr) => {
                    let string = self.eval_string(expr)?;
                    state.push_string(string)
                }
                Expr::Boolean(expr) => {
                    let boolean = self.eval_boolean(expr)?;
                    state.push_boolean(boolean)
                }
                Expr::None(_) => panic!("Cannot pass none to a callable"),
                Expr::Object(_, expr) => {
                    state.push_object(NullableReference::Some(self.eval_object(expr)?));
                }
            }
        }

        std::mem::swap(&mut state, &mut self.state);

        let callable = &self.callables[id];

        let result = self.interpret(&Rc::clone(callable));

        std::mem::swap(&mut state, &mut self.state);

        result
    }
}
