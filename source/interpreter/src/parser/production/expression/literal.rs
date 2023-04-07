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
            Some(TokenType::At) => match self.find_value("self".to_string().into()) {
                Some(Value::Object(_, obj_id)) => {
                    let property_name = match self.consume_token_type() {
                        Some(TokenType::Identifier(name)) => Rc::clone(name),
                        _ => {
                            self.error.add(ParserErrType::TempErrType(format!(
                                "Expected a property name after @"
                            )));
                            return Err(ParserStatus::End);
                        }
                    };

                    let result = if let Some(properties) = &self.current_properties {
                        match properties.get(&property_name) {
                            Some(Value::Integer(id)) => {
                                Ok(Expr::Integer(IntegerExpr::Property(obj_id, *id)))
                            }
                            Some(Value::Float(id)) => {
                                Ok(Expr::Float(FloatExpr::Property(obj_id, *id)))
                            }
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
                                self.error.add(ParserErrType::TempErrType(format!(
                                    "Property {} does not exist",
                                    property_name
                                )));
                                Err(ParserStatus::End)
                            }
                        }
                    } else {
                        self.error.add(ParserErrType::TempErrType(format!(
                            "Cannot access property {} as it is outside a method or constructor",
                            property_name,
                        )));
                        Err(ParserStatus::End)
                    };

                    if let Some(TokenType::Dot) = self.current_token_type() {
                        if let Ok(Expr::Object(class, expr)) = result {
                            self.method_call(class, expr)
                        } else {
                            self.error.add(ParserErrType::TempErrType(format!(
                                "Cannot call a method of a property that is not an object"
                            )));
                            Err(ParserStatus::End)
                        }
                    } else {
                        result
                    }
                }
                v => {
                    self.error.add(ParserErrType::TempErrType(format!(
                        "Self must be an object to use @ for property access, but it was type {:?}",
                        v,
                    )));
                    Err(ParserStatus::End)
                }
            },
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
                                self.error.add(ParserErrType::TempErrType(format!(
                                    "Function {} does not return an object",
                                    name,
                                )));
                                Err(ParserStatus::End)
                            }
                        } else {
                            result
                        }
                    }
                    // Calling a method of an object variable
                    Some(TokenType::Dot) => match self.find_value(Rc::clone(&name)) {
                        Some(Value::Object(class, id)) => {
                            self.method_call(class, ObjectExpr::Variable(id))
                        }
                        _ => {
                            self.error.add(ParserErrType::TempErrType(format!(
                                "Tried to call a method on variable {} that was not an object",
                                name
                            )));
                            Err(ParserStatus::End)
                        }
                    },
                    // Getting a variable
                    _ => match self.find_value(Rc::clone(&name)) {
                        Some(value) => Ok(self.get_variable_expr(&value)),
                        None => {
                            self.error.add(ParserErrType::TempErrType(format!(
                                "Could find the variable {}",
                                name,
                            )));
                            Err(ParserStatus::End)
                        }
                    },
                }
            }
            _ => {
                self.error.add(ParserErrType::ExpectedLiteralVariableCall(
                    self.tokens[self.current - 1].clone(),
                    self.tokens.get(self.current).cloned(),
                ));
                Err(ParserStatus::Unwind)
            }
        }
    }
}
