use crate::{
    native_function::*, parser::production::expression::prelude::*, parser::value::ValueType,
};

impl Parser {
    pub fn call(
        &mut self,
        name: &str,
        module: Option<String>,
        token_pos: usize,
    ) -> Result<Expr, ParserStatus> {
        debug_information!("call");
        self.current += 1;

        let mut arguments = vec![];

        match self.current_token_type() {
            Some(TokenType::RightParen) => {
                self.current += 1;
            }
            _ => loop {
                let argument = self.expression()?;

                arguments.push(argument);

                match self.tokens.get(self.current) {
                    Some(Token {
                        token_type: TokenType::Comma,
                        ..
                    }) => {
                        self.current += 1;
                        continue;
                    }
                    Some(Token {
                        token_type: TokenType::RightParen,
                        ..
                    }) => {
                        self.current += 1;
                        break;
                    }
                    t => {
                        self.error.add(ParserErrType::CallExpectedCommaOrRightParen(
                            self.tokens[self.current - 1].clone(),
                            t.cloned(),
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                };
            },
        }

        if let Some(module) = module {
            #[allow(dead_code)]
            enum InternalType {
                Integer,
                Float,
                Boolean,
                String,
                Printable,
            }

            let parameters = match (module.as_str(), name) {
                ("cli", "println") => vec![InternalType::Printable],
                ("cli", "print") => vec![InternalType::Printable],
                ("cli", "prompt") => vec![InternalType::String],
                ("gui", "add_heading") => vec![InternalType::String],
                ("gui", "add_paragraph") => vec![InternalType::String],
                ("gui", "add_hyperlink") => vec![InternalType::String],
                ("gui", "add_image") => vec![InternalType::String],
                ("gui", "add_button") => vec![InternalType::String],
                ("cli" | "gui", _) => {
                    self.error.add(ParserErrType::CallModuleFunctionNotFound(
                        self.tokens[token_pos].clone(),
                        name.to_string(),
                        module,
                    ));
                    return Err(ParserStatus::Unwind);
                }
                _ => {
                    self.error.add(ParserErrType::CallModuleNotFound(
                        self.tokens[token_pos - 2].clone(),
                        module,
                    ));
                    return Err(ParserStatus::Unwind);
                }
            };

            if arguments.len() != parameters.len() {
                self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                    self.tokens[token_pos].clone(),
                    arguments.len(),
                    parameters.len(),
                    name.to_string(),
                ));
                return Err(ParserStatus::Unwind);
            }

            let mut argument_error = false;

            for i in 0..arguments.len() {
                match (&arguments[i], &parameters[i]) {
                    (Expr::Integer(_), InternalType::Integer) => (),
                    (Expr::Float(_), InternalType::Float) => (),
                    (Expr::String(_), InternalType::String) => (),
                    (Expr::Boolean(_), InternalType::Boolean) => (),
                    (
                        Expr::Integer(_) | Expr::Float(_) | Expr::String(_) | Expr::Boolean(_),
                        InternalType::Printable,
                    ) => (),
                    (expr, _) => {
                        let expr_type = self.expr_type(expr);

                        self.error.add(ParserErrType::CallArgumentIncorrectType(
                            self.tokens[token_pos].clone(),
                            i,
                            expr_type,
                            name.to_string(),
                        ));

                        argument_error = true;
                    }
                }
            }

            if argument_error {
                return Err(ParserStatus::Unwind);
            }

            return Ok(match (module.as_str(), name) {
                ("cli", "println") => Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Cli(
                    match arguments.pop() {
                        Some(Expr::Integer(arg)) => {
                            CliFunctionNone::PrintLineInteger(Box::new(arg))
                        }
                        Some(Expr::Float(arg)) => CliFunctionNone::PrintLineFloat(Box::new(arg)),
                        Some(Expr::String(arg)) => CliFunctionNone::PrintLineString(Box::new(arg)),
                        Some(Expr::Boolean(arg)) => {
                            CliFunctionNone::PrintLineBoolean(Box::new(arg))
                        }
                        _ => unreachable!(),
                    },
                ))),
                ("cli", "print") => Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Cli(
                    match arguments.pop() {
                        Some(Expr::Integer(arg)) => CliFunctionNone::PrintInteger(Box::new(arg)),
                        Some(Expr::Float(arg)) => CliFunctionNone::PrintFloat(Box::new(arg)),
                        Some(Expr::String(arg)) => CliFunctionNone::PrintString(Box::new(arg)),
                        Some(Expr::Boolean(arg)) => CliFunctionNone::PrintBoolean(Box::new(arg)),
                        _ => unreachable!(),
                    },
                ))),
                ("cli", "prompt") => match arguments.pop() {
                    Some(Expr::String(argument)) => {
                        return Ok(Expr::String(StringExpr::NativeCall(
                            NativeFunctionString::Cli(CliFunctionString::Prompt(Box::new(
                                argument,
                            ))),
                        )));
                    }
                    _ => unreachable!(),
                },
                ("gui", "add_heading") => match arguments.pop() {
                    Some(Expr::String(value)) => {
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Gui(
                            GuiFunctionNone::AddHeading(Box::new(value)),
                        ))));
                    }
                    _ => unreachable!(),
                },
                ("gui", "add_paragraph") => match arguments.pop() {
                    Some(Expr::String(value)) => {
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Gui(
                            GuiFunctionNone::AddParagraph(Box::new(value)),
                        ))));
                    }
                    _ => unreachable!(),
                },
                ("gui", "add_hyperlink") => match arguments.pop() {
                    Some(Expr::String(value)) => {
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Gui(
                            GuiFunctionNone::AddHyperlink(Box::new(value)),
                        ))));
                    }
                    _ => unreachable!(),
                },
                ("gui", "add_image") => match arguments.pop() {
                    Some(Expr::String(value)) => {
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Gui(
                            GuiFunctionNone::AddImage(Box::new(value)),
                        ))));
                    }
                    _ => unreachable!(),
                },
                ("gui", "add_button") => match arguments.pop() {
                    Some(Expr::String(value)) => {
                        return Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Gui(
                            GuiFunctionNone::AddButton(Box::new(value)),
                        ))));
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            });
        }

        //Must be a zonkey call
        if let Some(call) = self.function_declarations.get(name) {
            if arguments.len() != call.parameters.len() {
                self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                    self.tokens[token_pos - 1].clone(),
                    arguments.len(),
                    call.parameters.len(),
                    name.to_string(),
                ));
                return Err(ParserStatus::Unwind);
            }

            // Check arguments evaluate to the same type as parameters
            for i in 0..arguments.len() {
                match (&arguments[i], &call.parameters[i].0) {
                    (Expr::Integer(_), ValueType::Integer) => (),
                    (Expr::Float(_), ValueType::Float) => (),
                    (Expr::String(_), ValueType::String) => (),
                    (Expr::Boolean(_), ValueType::Boolean) => (),
                    (Expr::Object(object_type, ..), ValueType::Class(class_type))
                        if object_type == class_type =>
                    {
                        ()
                    }
                    (expr, _) => {
                        let expr_type = self.expr_type(expr);

                        self.error.add(ParserErrType::CallArgumentIncorrectType(
                            self.tokens[token_pos - 1].clone(),
                            i,
                            expr_type,
                            name.to_string(),
                        ));
                    }
                }
            }

            return Ok(match &call.return_data_type {
                Some(ValueType::Integer) => Expr::Integer(IntegerExpr::Call(call.id, arguments)),
                Some(ValueType::Float) => Expr::Float(FloatExpr::Call(call.id, arguments)),
                Some(ValueType::String) => Expr::String(StringExpr::Call(call.id, arguments)),
                Some(ValueType::Boolean) => Expr::Boolean(BooleanExpr::Call(call.id, arguments)),
                Some(ValueType::Class(_)) => {
                    todo!()
                }
                None => return Ok(Expr::None(NoneExpr::Call(call.id, arguments))),
            });
        }

        self.error.add(ParserErrType::CallFunctionNotFound(
            self.tokens[token_pos - 1].clone(),
            name.to_string(),
        ));
        Err(ParserStatus::Unwind)
    }
}
