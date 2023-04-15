use crate::{parser::production::expression::prelude::*, parser::value::Value};
use std::rc::Rc;

impl Parser {
    fn find_value(&self, name: Rc<String>) -> Option<Value> {
        for scope in self.value_stack.iter().rev() {
            if let Some(value) = scope.get(&name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn get_variable_expr(&self, value: &Value) -> Expr {
        match value {
            Value::Integer(id) => Expr::Integer(IntegerExpr::Variable(*id)),
            Value::Float(id) => Expr::Float(FloatExpr::Variable(*id)),
            Value::String(id) => Expr::String(StringExpr::Variable(*id)),
            Value::Boolean(id) => Expr::Boolean(BooleanExpr::Variable(*id)),
            Value::Object(class, id) => Expr::Object(Rc::clone(class), ObjectExpr::Variable(*id)),
        }
    }

    pub fn literal(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("literal");

        match self.consume_token_type() {
            Some(TokenType::Integer(val)) => Ok(Expr::Integer(IntegerExpr::Literal(*val))),
            Some(TokenType::Float(val)) => Ok(Expr::Float(FloatExpr::Literal(*val))),
            Some(TokenType::String(val)) => Ok(Expr::String(StringExpr::Literal(Rc::clone(val)))),
            Some(TokenType::Boolean(val)) => Ok(Expr::Boolean(BooleanExpr::Literal(*val))),
            Some(TokenType::LeftParen) => self.grouping(),
            // Getting a property
            Some(TokenType::At) => {
                let property_name = match self.consume_token_type() {
                    Some(TokenType::Identifier(name)) => Rc::clone(name),
                    _ => {
                        self.error.add(ParserErrType::PropertyAccessorExpectedName(
                            self.tokens[self.current - 2].clone(),
                            self.tokens.get(self.current - 1).cloned(),
                        ));
                        return Err(ParserStatus::End);
                    }
                };

                let result = if let Some(properties) = &self.current_properties {
                    let obj_id = match self.find_value("self".to_string().into()) {
                        Some(Value::Object(_, obj_id)) => obj_id,
                        _ => {
                            panic!("Self should always be an object.")
                        }
                    };

                    match properties.get(&property_name) {
                        Some(Value::Integer(id)) => {
                            Ok(Expr::Integer(IntegerExpr::Property(obj_id, *id)))
                        }
                        Some(Value::Float(id)) => Ok(Expr::Float(FloatExpr::Property(obj_id, *id))),
                        Some(Value::String(id)) => {
                            Ok(Expr::String(StringExpr::Property(obj_id, *id)))
                        }
                        Some(Value::Boolean(id)) => {
                            Ok(Expr::Boolean(BooleanExpr::Property(obj_id, *id)))
                        }
                        Some(Value::Object(class, id)) => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::Property(obj_id, *id),
                        )),
                        _ => {
                            self.error.add(ParserErrType::PropertyNotFound(
                                self.tokens[self.current - 1].clone(),
                                property_name.to_string(),
                            ));
                            Err(ParserStatus::End)
                        }
                    }
                } else {
                    self.error.add(ParserErrType::PropertyAccessorOutsideClass(
                        self.tokens[self.current - 1].clone(),
                        property_name.to_string(),
                    ));
                    Err(ParserStatus::End)
                };

                if let Some(TokenType::Dot) = self.current_token_type() {
                    if let Ok(Expr::Object(class, expr)) = result {
                        self.method_call(class, expr)
                    } else {
                        self.error.add(ParserErrType::MethodCallNotObject(
                            self.tokens[self.current].clone(),
                            self.expr_type(&result?),
                        ));
                        Err(ParserStatus::End)
                    }
                } else {
                    result
                }
            }
            Some(TokenType::Identifier(name)) => {
                let name = Rc::clone(&name);
                match self.current_token_type() {
                    // Calling a function
                    Some(TokenType::LeftParen) => {
                        let result = self.function_call(Rc::clone(&name));

                        // Calling a method on a function result
                        if let Some(TokenType::Dot) = self.current_token_type() {
                            if let Ok(Expr::Object(class, expr)) = result {
                                self.method_call(class, expr)
                            } else {
                                self.error.add(ParserErrType::MethodCallNotObject(
                                    self.tokens[self.current].clone(),
                                    self.expr_type(&result?),
                                ));
                                Err(ParserStatus::Unwind)
                            }
                        } else {
                            result
                        }
                    }

                    // Creating an array of objects
                    Some(TokenType::LeftBracket) => self.array_constructor(Rc::clone(&name)),

                    // Calling a method of an object variable
                    Some(TokenType::Dot) => match self.find_value(Rc::clone(&name)) {
                        Some(Value::Object(class, id)) => {
                            self.method_call(class, ObjectExpr::Variable(id))
                        }
                        Some(value) => {
                            self.error.add(ParserErrType::MethodCallNotObject(
                                self.tokens[self.current].clone(),
                                Some(value.to_value_type()),
                            ));
                            Err(ParserStatus::Unwind)
                        }
                        None => {
                            self.error.add(ParserErrType::VariableNotFound(
                                self.tokens[self.current - 1].clone(),
                                name.to_string(),
                            ));
                            Err(ParserStatus::Unwind)
                        }
                    },
                    // Getting a variable
                    _ => match self.find_value(Rc::clone(&name)) {
                        Some(value) => Ok(self.get_variable_expr(&value)),
                        None => {
                            self.error.add(ParserErrType::VariableNotFound(
                                self.tokens[self.current - 1].clone(),
                                name.to_string(),
                            ));
                            Err(ParserStatus::Unwind)
                        }
                    },
                }
            }
            _ => {
                self.error.add(ParserErrType::ExpectedValue(
                    self.tokens[self.current - 2].clone(),
                    self.tokens.get(self.current - 1).cloned(),
                ));
                self.current -= 1;
                Err(ParserStatus::Unwind)
            }
        }
    }
}
