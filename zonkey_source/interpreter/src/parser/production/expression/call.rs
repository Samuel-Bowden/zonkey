use crate::{
    parser::production::expression::prelude::*,
    parser::{
        declaration::{CallableDeclaration, CallableType},
        value::ValueType,
    },
    prelude::*,
};
use std::rc::Rc;

impl Parser {
    pub fn call(
        &mut self,
        object: Option<Expr>,
        name: Rc<String>,
        call: Rc<CallableDeclaration>,
    ) -> Result<Expr, ParserStatus> {
        debug_information!("call");

        let token_pos = self.current - 1;

        let mut arguments = vec![];

        if let Some(object) = object {
            arguments.push(object)
        }

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

        if arguments.len() != call.parameters.len() {
            self.error.add(ParserErrType::CallIncorrectArgumentsNum(
                self.tokens[token_pos - 1].clone(),
                arguments.len(),
                call.parameters.len(),
                name.to_string(),
            ));
            return Err(ParserStatus::Unwind);
        }

        match call.callable_type {
            CallableType::Native => match name.as_str() {
                "print" => Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Print(
                    Box::new(arguments.remove(0)),
                    false,
                )))),
                "println" => Ok(Expr::None(NoneExpr::NativeCall(NativeFunctionNone::Print(
                    Box::new(arguments.remove(0)),
                    true,
                )))),
                "prompt" => {
                    if let Expr::String(expr) = arguments.remove(0) {
                        Ok(Expr::String(StringExpr::NativeCall(
                            NativeFunctionString::Prompt(Box::new(expr)),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "wait_for_event" => Ok(Expr::Boolean(BooleanExpr::NativeCall(
                    NativeFunctionBoolean::WaitForEvent,
                ))),
                "add_button" => {
                    if let Expr::Object(_, button) = arguments.remove(0) {
                        Ok(Expr::None(NoneExpr::NativeCall(
                            NativeFunctionNone::AddButton(button),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "add_heading" => {
                    if let Expr::Object(_, heading) = arguments.remove(0) {
                        Ok(Expr::None(NoneExpr::NativeCall(
                            NativeFunctionNone::AddHeading(heading),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "add_paragraph" => {
                    if let Expr::Object(_, paragraph) = arguments.remove(0) {
                        Ok(Expr::None(NoneExpr::NativeCall(
                            NativeFunctionNone::AddParagraph(paragraph),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "add_hyperlink" => {
                    if let Expr::Object(_, hyperlink) = arguments.remove(0) {
                        Ok(Expr::None(NoneExpr::NativeCall(
                            NativeFunctionNone::AddHyperlink(hyperlink),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "add_input" => {
                    if let Expr::Object(_, input) = arguments.remove(0) {
                        Ok(Expr::None(NoneExpr::NativeCall(
                            NativeFunctionNone::AddInput(input),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "text" => {
                    if let (Expr::Object(_, button), Expr::String(text)) =
                        (arguments.remove(0), arguments.remove(0))
                    {
                        Ok(Expr::None(NoneExpr::NativeCall(
                            NativeFunctionNone::ButtonText(button, Box::new(text)),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "clicked" => {
                    if let Expr::Object(_, button) = arguments.remove(0) {
                        Ok(Expr::Boolean(BooleanExpr::NativeCall(
                            NativeFunctionBoolean::ButtonClicked(button),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "confirmed" => {
                    if let Expr::Object(_, button) = arguments.remove(0) {
                        Ok(Expr::Boolean(BooleanExpr::NativeCall(
                            NativeFunctionBoolean::ButtonClicked(button),
                        )))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "Button" => {
                    if let Expr::String(text) = arguments.remove(0) {
                        Ok(Expr::Object(
                            Rc::new("Button".to_string()),
                            ObjectExpr::NativeCall(NativeFunctionObject::ButtonConstructor(
                                Box::new(text),
                            )),
                        ))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "Heading" => {
                    if let Expr::String(text) = arguments.remove(0) {
                        Ok(Expr::Object(
                            Rc::new("Heading".to_string()),
                            ObjectExpr::NativeCall(NativeFunctionObject::HeadingConstructor(
                                Box::new(text),
                            )),
                        ))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "Paragraph" => {
                    if let Expr::String(text) = arguments.remove(0) {
                        Ok(Expr::Object(
                            Rc::new("Paragraph".to_string()),
                            ObjectExpr::NativeCall(NativeFunctionObject::ParagraphConstructor(
                                Box::new(text),
                            )),
                        ))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "Hyperlink" => {
                    if let (Expr::String(text), Expr::String(link)) =
                        (arguments.remove(0), arguments.remove(0))
                    {
                        Ok(Expr::Object(
                            Rc::new("Hyperlink".to_string()),
                            ObjectExpr::NativeCall(NativeFunctionObject::HyperlinkConstructor(
                                Box::new(text),
                                Box::new(link),
                            )),
                        ))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                "Input" => {
                    if let Expr::String(text) = arguments.remove(0) {
                        Ok(Expr::Object(
                            Rc::new("Input".to_string()),
                            ObjectExpr::NativeCall(NativeFunctionObject::InputConstructor(
                                Box::new(text),
                            )),
                        ))
                    } else {
                        unreachable!("Already tested type")
                    }
                }
                _ => todo!(),
            },
            CallableType::Zonkey(id) => {
                // Check arguments evaluate to the same type as parameters
                for i in 0..arguments.len() {
                    match (&arguments[i], &call.parameters[i]) {
                        (Expr::Integer(_), ValueType::Integer) => (),
                        (Expr::Float(_), ValueType::Float) => (),
                        (Expr::String(_), ValueType::String) => (),
                        (Expr::Boolean(_), ValueType::Boolean) => (),
                        (_, ValueType::Any) => (),
                        (Expr::Object(class, _), ValueType::Class(name)) => if class == name {},
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

                Ok(match &call.return_type {
                    Some(ValueType::Integer) => Expr::Integer(IntegerExpr::Call(id, arguments)),
                    Some(ValueType::Float) => Expr::Float(FloatExpr::Call(id, arguments)),
                    Some(ValueType::String) => Expr::String(StringExpr::Call(id, arguments)),
                    Some(ValueType::Boolean) => Expr::Boolean(BooleanExpr::Call(id, arguments)),
                    Some(ValueType::Any) => unreachable!("Zonkey code cannot use the Any type"),
                    Some(ValueType::Class(class)) => {
                        Expr::Object(Rc::clone(class), ObjectExpr::Call(id, arguments))
                    }
                    None => Expr::None(NoneExpr::Call(id, arguments)),
                })
            }
        }
    }
}
