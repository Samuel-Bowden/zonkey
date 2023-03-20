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
                Some(Value::Object(class, obj_id)) => {
                    let property_name = match self.consume_token_type() {
                        Some(TokenType::Identifier(name)) => Rc::clone(name),
                        _ => panic!("Expected method name"),
                    };

                    let result = match self
                        .class_declarations
                        .get(&class)
                        .unwrap()
                        .properties
                        .get(&property_name)
                    {
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
                        _ => panic!("Property does not exist"),
                    };

                    if let Some(TokenType::Dot) = self.current_token_type() {
                        if let Ok(Expr::Object(class, expr)) = result {
                            self.method_call(class, expr)
                        } else {
                            panic!("Cannot call method of a property that is not an object");
                        }
                    } else {
                        result
                    }
                }
                v => {
                    panic!("Self must be an object, but was {:?}", v);
                }
            },
            Some(TokenType::Identifier(name)) => {
                let name = Rc::clone(&name);
                match self.current_token_type() {
                    // Calling a function
                    Some(TokenType::LeftParen) => {
                        let result = self.function_call(name);

                        // Calling a method on a function result
                        if let Some(TokenType::Dot) = self.current_token_type() {
                            if let Ok(Expr::Object(class, expr)) = result {
                                self.method_call(class, expr)
                            } else {
                                panic!("Function did not return an object");
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
                            panic!("Variable {} is not an object", name);
                        }
                    },
                    // Getting a variable
                    _ => match self.find_value(Rc::clone(&name)) {
                        Some(value) => Ok(self.get_variable_expr(&value)),
                        None => {
                            panic!("Value {} not found", name);
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
