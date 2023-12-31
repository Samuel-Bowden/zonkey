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
        let mut result = self.method_call_handler(class, object);

        while let Some(TokenType::Dot) = self.current_token_type() {
            match result {
                Ok(Expr::Object(class, expr)) => {
                    result = self.method_call_handler(class, expr);
                }
                Err(_) => return result,
                Ok(value) => {
                    self.error.add(ParserErrType::MethodCallNotObject(
                        self.tokens[self.current].clone(),
                        self.expr_type(&value),
                    ));
                    return Err(ParserStatus::Unwind);
                }
            }
        }

        return result;
    }

    fn method_call_handler(
        &mut self,
        class: Rc<String>,
        object: ObjectExpr,
    ) -> Result<Expr, ParserStatus> {
        debug_information!("method_call_handler");

        let token_pos = self.current;
        self.current += 1;

        let name = match self.consume_token_type() {
            Some(TokenType::Identifier(name)) => Rc::clone(name),
            _ => {
                self.error.add(ParserErrType::MethodCallExpectedName(
                    self.tokens[self.current - 2].clone(),
                    self.tokens.get(self.current - 1).cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };

        match self.consume_token_type() {
            Some(TokenType::LeftParen) => (),
            _ => {
                self.error.add(ParserErrType::MethodCallExpectedLeftParen(
                    self.tokens[self.current - 2].clone(),
                    self.tokens.get(self.current - 1).cloned(),
                ));
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
                    self.tokens[token_pos + 1].clone(),
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
                    (Expr::Object(class, _), ValueType::Element)
                        if matches!(
                            class.as_str(),
                            "Button" | "Text" | "Hyperlink" | "Input" | "Row" | "Column" | "Image"
                        ) => {}
                    (Expr::Object(class, _), ValueType::Class(name)) if class == name => (),
                    (expr, _) => {
                        let expr_type = self.expr_type(expr);

                        failed = true;

                        self.error.add(ParserErrType::CallArgumentIncorrectType(
                            self.tokens[token_pos + 1].clone(),
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

            match call.callable_type {
                CallableType::Native => match class.as_str() {
                    "Hyperlink" => match name.as_str() {
                        "add_argument" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::HyperlinkAddArg(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
                            )),
                        )),
                        _ => unreachable!(),
                    },
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
                        "get_text" => Ok(Expr::String(StringExpr::NativeCall(
                            NativeCallString::GetButtonText(object),
                        ))),
                        "set_background_colour" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ButtonSetBackgroundColour(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
                            )),
                        )),
                        "set_text_colour" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ButtonSetTextColour(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
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
                        "set_text" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::InputSetText(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
                            )),
                        )),
                        _ => unreachable!(),
                    },
                    "Text" => match name.as_str() {
                        "set_text" => Ok(Expr::Object(
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
                                Box::new(arguments.remove(0).to_string_expr()),
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
                        "set_max_width" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::PageSetMaxWidth(
                                Box::new(object),
                                arguments.remove(0).to_float_expr(),
                            )),
                        )),
                        "center" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::PageCenter(Box::new(object))),
                        )),
                        "add" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::PageAddElement(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_object_expr()),
                            )),
                        )),
                        "remove" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::PageRemoveElement(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_object_expr()),
                            )),
                        )),
                        "set_background_colour" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::PageSetBackgroundColour(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
                            )),
                        )),
                        "set_text_colour" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::PageSetTextColour(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_string_expr()),
                            )),
                        )),
                        _ => unreachable!(),
                    },
                    "Row" => match name.as_str() {
                        "add" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::RowAddElement(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_object_expr()),
                            )),
                        )),
                        "remove" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::RowRemoveElement(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_object_expr()),
                            )),
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
                            ObjectExpr::NativeCall(NativeCallObject::ColumnAddElement(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_object_expr()),
                            )),
                        )),
                        "remove" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ColumnRemoveElement(
                                Box::new(object),
                                Box::new(arguments.remove(0).to_object_expr()),
                            )),
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
                    "Image" => match name.as_str() {
                        "set_max_width" => Ok(Expr::Object(
                            Rc::clone(&class),
                            ObjectExpr::NativeCall(NativeCallObject::ImageSetMaxWidth(
                                Box::new(object),
                                arguments.remove(0).to_float_expr(),
                            )),
                        )),
                        _ => unreachable!(),
                    },
                    array_object => {
                        match (name.as_str(), &array_object[1..array_object.len() - 1]) {
                            ("get", "Integer") => Ok(Expr::Integer(IntegerExpr::NativeCall(
                                NativeCallInteger::IntegerArrayGet(
                                    Box::new(object),
                                    Box::new(arguments.remove(0).to_integer_expr()),
                                    self.tokens[token_pos + 1].clone(),
                                ),
                            ))),
                            ("push", "Integer") => Ok(Expr::Object(
                                Rc::clone(&class),
                                ObjectExpr::NativeCall(NativeCallObject::IntegerArrayPush(
                                    Box::new(object),
                                    arguments.remove(0).to_integer_expr(),
                                )),
                            )),
                            ("remove", "Integer") => Ok(Expr::Integer(IntegerExpr::NativeCall(
                                NativeCallInteger::IntegerArrayRemove(
                                    Box::new(object),
                                    Box::new(arguments.remove(0).to_integer_expr()),
                                    self.tokens[token_pos + 1].clone(),
                                ),
                            ))),
                            ("get", "Float") => Ok(Expr::Float(FloatExpr::NativeCall(
                                NativeCallFloat::FloatArrayGet(
                                    Box::new(object),
                                    Box::new(arguments.remove(0).to_integer_expr()),
                                    self.tokens[token_pos + 1].clone(),
                                ),
                            ))),
                            ("push", "Float") => Ok(Expr::Object(
                                Rc::clone(&class),
                                ObjectExpr::NativeCall(NativeCallObject::FloatArrayPush(
                                    Box::new(object),
                                    arguments.remove(0).to_float_expr(),
                                )),
                            )),
                            ("remove", "Float") => Ok(Expr::Float(FloatExpr::NativeCall(
                                NativeCallFloat::FloatArrayRemove(
                                    Box::new(object),
                                    Box::new(arguments.remove(0).to_integer_expr()),
                                    self.tokens[token_pos + 1].clone(),
                                ),
                            ))),
                            ("get", "Boolean") => Ok(Expr::Boolean(BooleanExpr::NativeCall(
                                NativeCallBoolean::BooleanArrayGet(
                                    Box::new(object),
                                    arguments.remove(0).to_integer_expr(),
                                    self.tokens[token_pos + 1].clone(),
                                ),
                            ))),
                            ("push", "Boolean") => Ok(Expr::Object(
                                Rc::clone(&class),
                                ObjectExpr::NativeCall(NativeCallObject::BooleanArrayPush(
                                    Box::new(object),
                                    Box::new(arguments.remove(0).to_boolean_expr()),
                                )),
                            )),
                            ("remove", "Boolean") => Ok(Expr::Boolean(BooleanExpr::NativeCall(
                                NativeCallBoolean::BooleanArrayRemove(
                                    Box::new(object),
                                    arguments.remove(0).to_integer_expr(),
                                    self.tokens[token_pos + 1].clone(),
                                ),
                            ))),
                            ("get", "String") => Ok(Expr::String(StringExpr::NativeCall(
                                NativeCallString::StringArrayGet(
                                    Box::new(object),
                                    arguments.remove(0).to_integer_expr(),
                                    self.tokens[token_pos + 1].clone(),
                                ),
                            ))),
                            ("push", "String") => Ok(Expr::Object(
                                Rc::clone(&class),
                                ObjectExpr::NativeCall(NativeCallObject::StringArrayPush(
                                    Box::new(object),
                                    Box::new(arguments.remove(0).to_string_expr()),
                                )),
                            )),
                            ("remove", "String") => Ok(Expr::String(StringExpr::NativeCall(
                                NativeCallString::StringArrayRemove(
                                    Box::new(object),
                                    arguments.remove(0).to_integer_expr(),
                                    self.tokens[token_pos + 1].clone(),
                                ),
                            ))),
                            ("get", object_name) => Ok(Expr::Object(
                                Rc::new(object_name.into()),
                                ObjectExpr::NativeCall(NativeCallObject::ObjectArrayGet(
                                    Box::new(object),
                                    arguments.remove(0).to_integer_expr(),
                                    self.tokens[token_pos + 1].clone(),
                                )),
                            )),
                            ("push", _) => Ok(Expr::Object(
                                Rc::clone(&class),
                                ObjectExpr::NativeCall(NativeCallObject::ObjectArrayPush(
                                    Box::new(object),
                                    Box::new(arguments.remove(0).to_object_expr()),
                                )),
                            )),
                            ("remove", object_name) => Ok(Expr::Object(
                                Rc::new(object_name.into()),
                                ObjectExpr::NativeCall(NativeCallObject::ObjectArrayRemove(
                                    Box::new(object),
                                    arguments.remove(0).to_integer_expr(),
                                    self.tokens[token_pos + 1].clone(),
                                )),
                            )),
                            ("len", _) => Ok(Expr::Integer(IntegerExpr::NativeCall(
                                NativeCallInteger::ArrayLength(Box::new(object)),
                            ))),
                            p => unreachable!("Expected an array object but found {:?}", p),
                        }
                    }
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
                        Some(ValueType::Printable | ValueType::Element | ValueType::Generic) => {
                            unreachable!("Zonkey code cannot use these types")
                        }
                    }
                }
            }
        } else {
            self.error.add(ParserErrType::MethodCallNotFound(
                self.tokens[token_pos + 1].clone(),
                name.to_string(),
                class.to_string(),
            ));
            Err(ParserStatus::Unwind)
        }
    }
}
