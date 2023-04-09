use crate::{
    parser::production::expression::prelude::*,
    parser::{declaration::CallableType, value::ValueType},
    standard_prelude::calls::*,
};
use std::rc::Rc;

impl Parser {
    pub fn function_call(&mut self, name: Rc<String>) -> Result<Expr, ParserStatus> {
        debug_information!("function_call");

        let token_pos = self.current;
        self.current += 1;

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

        if let Some(call) = self.function_declarations.get(&name) {
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

            match call.callable_type {
                CallableType::Native => match name.as_str() {
                    "print" => Ok(Expr::None(NoneExpr::NativeCall(NativeCallNone::Print(
                        Box::new(arguments.remove(0)),
                        false,
                    )))),
                    "println" => Ok(Expr::None(NoneExpr::NativeCall(NativeCallNone::Print(
                        Box::new(arguments.remove(0)),
                        true,
                    )))),
                    "prompt" => Ok(Expr::String(StringExpr::NativeCall(
                        NativeCallString::Prompt(Box::new(arguments.remove(0).to_string_expr())),
                    ))),
                    "wait_for_event" => Ok(Expr::Boolean(BooleanExpr::NativeCall(
                        NativeCallBoolean::WaitForEvent,
                    ))),
                    "sleep" => Ok(Expr::None(NoneExpr::NativeCall(NativeCallNone::Sleep(
                        arguments.remove(0).to_integer_expr(),
                    )))),
                    "Page" => Ok(Expr::Object(
                        Rc::new("Page".to_string()),
                        ObjectExpr::NativeCall(NativeCallObject::PageConstructor),
                    )),
                    "Button" => Ok(Expr::Object(
                        Rc::new("Button".to_string()),
                        ObjectExpr::NativeCall(NativeCallObject::ButtonConstructor(Box::new(
                            arguments.remove(0).to_string_expr(),
                        ))),
                    )),
                    "Text" => Ok(Expr::Object(
                        Rc::new("Text".to_string()),
                        ObjectExpr::NativeCall(NativeCallObject::TextConstructor(Box::new(
                            arguments.remove(0).to_string_expr(),
                        ))),
                    )),
                    "Hyperlink" => Ok(Expr::Object(
                        Rc::new("Hyperlink".to_string()),
                        ObjectExpr::NativeCall(NativeCallObject::HyperlinkConstructor(
                            Box::new(arguments.remove(0).to_string_expr()),
                            Box::new(arguments.remove(0).to_string_expr()),
                        )),
                    )),
                    "Input" => Ok(Expr::Object(
                        Rc::new("Input".to_string()),
                        ObjectExpr::NativeCall(NativeCallObject::InputConstructor(Box::new(
                            arguments.remove(0).to_string_expr(),
                        ))),
                    )),
                    "Row" => Ok(Expr::Object(
                        Rc::new("Row".to_string()),
                        ObjectExpr::NativeCall(NativeCallObject::RowConstructor),
                    )),
                    "Column" => Ok(Expr::Object(
                        Rc::new("Column".to_string()),
                        ObjectExpr::NativeCall(NativeCallObject::ColumnConstructor),
                    )),
                    "Image" => Ok(Expr::Object(
                        Rc::new("Image".to_string()),
                        ObjectExpr::NativeCall(NativeCallObject::ImageConstructor(Box::new(
                            arguments.remove(0).to_string_expr(),
                        ))),
                    )),
                    _ => unreachable!(),
                },
                CallableType::Zonkey(id) => Ok(match &call.return_type {
                    Some(ValueType::Integer) => Expr::Integer(IntegerExpr::Call(id, arguments)),
                    Some(ValueType::Float) => Expr::Float(FloatExpr::Call(id, arguments)),
                    Some(ValueType::String) => Expr::String(StringExpr::Call(id, arguments)),
                    Some(ValueType::Boolean) => Expr::Boolean(BooleanExpr::Call(id, arguments)),
                    Some(ValueType::Class(class)) => {
                        Expr::Object(Rc::clone(class), ObjectExpr::Call(id, arguments))
                    }
                    None => Expr::None(NoneExpr::Call(id, arguments)),
                    Some(ValueType::Any | ValueType::Element) => {
                        unreachable!("Zonkey code cannot use these types")
                    }
                }),
            }
        } else {
            self.error.add(ParserErrType::CallNotFound(
                self.tokens[token_pos - 1].clone(),
                name.to_string(),
            ));
            Err(ParserStatus::Unwind)
        }
    }
}
