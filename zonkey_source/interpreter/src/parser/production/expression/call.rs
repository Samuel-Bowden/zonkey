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
