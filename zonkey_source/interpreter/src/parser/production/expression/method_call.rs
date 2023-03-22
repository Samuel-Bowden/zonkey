use crate::{
    parser::production::expression::prelude::*,
    parser::{declaration::CallableType, value::ValueType},
    standard_prelude::calls::*,
};
use std::rc::Rc;

impl Parser {
    pub fn method_call(
        &mut self,
        class: Rc<String>,
        object: ObjectExpr,
    ) -> Result<Expr, ParserStatus> {
        debug_information!("method_call");

        let token_pos = self.current;
        self.current += 1;

        let name = match self.consume_token_type() {
            Some(TokenType::Identifier(name)) => Rc::clone(name),
            _ => {
                self.error.add(ParserErrType::TempErrType(format!(
                    "Expected method name after ."
                )));
                return Err(ParserStatus::End);
            }
        };

        match self.consume_token_type() {
            Some(TokenType::LeftParen) => (),
            _ => {
                self.error.add(ParserErrType::TempErrType(format!(
                    "Expected left paren after method name {}",
                    name,
                )));
                return Err(ParserStatus::End);
            }
        };

        let mut arguments = vec![];

        match self.current_token_type() {
            Some(TokenType::RightParen) => {
                self.current += 1;
            }
            _ => loop {
                let argument = self.expression()?;

                arguments.push(argument);

                match self.consume_token_type() {
                    Some(TokenType::Comma) => continue,
                    Some(TokenType::RightParen) => break,
                    _ => {
                        self.error.add(ParserErrType::CallExpectedCommaOrRightParen(
                            self.tokens[self.current - 2].clone(),
                            self.tokens.get(self.current - 1).cloned(),
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                };
            },
        }

        if let Some(call) = self
            .class_declarations
            .get(&class)
            .unwrap()
            .methods
            .get(&name)
        {
            if arguments.len() != call.parameters.len() {
                self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                    self.tokens[token_pos - 1].clone(),
                    arguments.len(),
                    call.parameters.len(),
                    name.to_string(),
                ));
                return Err(ParserStatus::Unwind);
            }

            let mut failed = false;

            // Check arguments evaluate to the same type as parameters
            for i in 0..arguments.len() {
                match (&arguments[i], &call.parameters[i]) {
                    (Expr::Integer(_), ValueType::Integer) => (),
                    (Expr::Float(_), ValueType::Float) => (),
                    (Expr::String(_), ValueType::String) => (),
                    (Expr::Boolean(_), ValueType::Boolean) => (),
                    (_, ValueType::Any) => (),
                    (Expr::Object(class, _), ValueType::Element)
                        if matches!(
                            class.as_str(),
                            "Button" | "Text" | "Hyperlink" | "Input" | "Row" | "Column"
                        ) => {}
                    (Expr::Object(class, _), ValueType::Class(name)) if class == name => {}
                    (expr, _) => {
                        let expr_type = self.expr_type(expr);

                        failed = true;

                        self.error.add(ParserErrType::CallArgumentIncorrectType(
                            self.tokens[token_pos - 1].clone(),
                            i,
                            expr_type,
                            name.to_string(),
                        ));
                    }
                }
            }

            if failed {
                return Err(ParserStatus::Unwind);
            }

            let result = match call.callable_type {
                CallableType::Native => match class.as_str() {
                    "Button" => match name.as_str() {
                        "clicked" => Ok(Expr::Boolean(BooleanExpr::NativeCall(
                            NativeCallBoolean::ButtonClicked(object),
                        ))),
                        "set_text" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ButtonSetText(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
                            )),
                        )),
                        "set_background_colour" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ButtonSetBackgroundColour(
                                Box::new(object),
                                arguments.remove(0).to_float_expr(),
                                arguments.remove(0).to_float_expr(),
                                arguments.remove(0).to_float_expr(),
                            )),
                        )),
                        "set_padding" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ButtonSetPadding(
                                Box::new(object),
                                arguments.remove(0).to_float_expr(),
                                arguments.remove(0).to_float_expr(),
                            )),
                        )),
                        "set_width_fill" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ButtonSetWidthFill(Box::new(
                                object,
                            ))),
                        )),
                        _ => unreachable!(),
                    },
                    "Input" => match name.as_str() {
                        "confirmed" => Ok(Expr::Boolean(BooleanExpr::NativeCall(
                            NativeCallBoolean::InputConfirmed(object),
                        ))),
                        "get_text" => Ok(Expr::String(StringExpr::NativeCall(
                            NativeCallString::GetInputText(object),
                        ))),
                        _ => unreachable!(),
                    },
                    "Text" => match name.as_str() {
                        "set_value" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::TextSetValue(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
                            )),
                        )),
                        "set_size" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::TextSetSize(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_float_expr()),
                            )),
                        )),
                        "set_colour" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::TextSetColour(
                                Box::new(object),
                                arguments.remove(0).to_float_expr(),
                                arguments.remove(0).to_float_expr(),
                                arguments.remove(0).to_float_expr(),
                            )),
                        )),
                        _ => unreachable!(),
                    },
                    "Page" => match name.as_str() {
                        "set_title" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::PageSetTitle(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
                            )),
                        )),
                        "add" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(
                                if let Expr::Object(_, expr) = arguments.remove(0) {
                                    NativeCallObject::PageAddElement(
                                        Box::new(object),
                                        Box::new(expr),
                                    )
                                } else {
                                    unreachable!()
                                },
                            ),
                        )),
                        "set_background_colour" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::PageSetBackgroundColour(
                                Box::new(object),
                                arguments.remove(0).to_float_expr(),
                                arguments.remove(0).to_float_expr(),
                                arguments.remove(0).to_float_expr(),
                            )),
                        )),
                        _ => unreachable!(),
                    },
                    "Row" => match name.as_str() {
                        "add" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(
                                if let Expr::Object(_, expr) = arguments.remove(0) {
                                    NativeCallObject::RowAddElement(
                                        Box::new(object),
                                        Box::new(expr),
                                    )
                                } else {
                                    unreachable!()
                                },
                            ),
                        )),
                        "center" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::RowCenter(Box::new(object))),
                        )),
                        _ => unreachable!(),
                    },
                    "Column" => match name.as_str() {
                        "add" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(
                                if let Expr::Object(_, expr) = arguments.remove(0) {
                                    NativeCallObject::ColumnAddElement(
                                        Box::new(object),
                                        Box::new(expr),
                                    )
                                } else {
                                    unreachable!()
                                },
                            ),
                        )),
                        "set_max_width" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ColumnSetMaxWidth(
                                Box::new(object),
                                arguments.remove(0).to_float_expr(),
                            )),
                        )),
                        _ => unreachable!(),
                    },
                    _ => todo!(),
                },
                CallableType::Zonkey(id) => {
                    arguments.insert(0, Expr::Object(Rc::clone(&class), object));

                    match &call.return_type {
                        Some(ValueType::Integer) => {
                            Ok(Expr::Integer(IntegerExpr::Call(id, arguments)))
                        }
                        Some(ValueType::Float) => Ok(Expr::Float(FloatExpr::Call(id, arguments))),
                        Some(ValueType::String) => {
                            Ok(Expr::String(StringExpr::Call(id, arguments)))
                        }
                        Some(ValueType::Boolean) => {
                            Ok(Expr::Boolean(BooleanExpr::Call(id, arguments)))
                        }
                        Some(ValueType::Class(class)) => Ok(Expr::Object(
                            Rc::clone(class),
                            ObjectExpr::Call(id, arguments),
                        )),
                        None => Ok(Expr::None(NoneExpr::Call(id, arguments))),
                        Some(ValueType::Any | ValueType::Element) => {
                            unreachable!("Zonkey code cannot use these types")
                        }
                    }
                }
            };

            // Calling a method on the result of a method
            if let Some(TokenType::Dot) = self.current_token_type() {
                match result {
                    Ok(Expr::Object(class, expr)) => self.method_call(class, expr),
                    Err(_) => result,
                    _ => {
                        self.error.add(ParserErrType::TempErrType(format!(
                            "Method {} did not return an object",
                            name,
                        )));
                        return Err(ParserStatus::End);
                    }
                }
            } else {
                result
            }
        } else {
            self.error.add(ParserErrType::CallFunctionNotFound(
                self.tokens[token_pos - 1].clone(),
                name.to_string(),
            ));
            Err(ParserStatus::Unwind)
        }
    }
}
