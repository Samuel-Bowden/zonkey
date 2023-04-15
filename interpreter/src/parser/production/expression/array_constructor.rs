use std::rc::Rc;

use crate::{
    parser::{production::expression::prelude::*, value::print_type},
    standard_prelude::{calls::NativeCallObject, classes::array},
};

impl Parser {
    pub fn array_constructor(&mut self, value_type: Rc<String>) -> Result<Expr, ParserStatus> {
        debug_information!("array_constructor");

        let value_type_pos = self.current - 1;

        let value_type = match value_type.as_str() {
            "Integer" => ValueType::Integer,
            "Float" => ValueType::Float,
            "String" => ValueType::String,
            "Boolean" => ValueType::Boolean,
            _ => match self.class_declarations.get(&value_type) {
                Some(_) => ValueType::Class(Rc::clone(&value_type)),
                None => {
                    self.error.add(ParserErrType::ClassNotFound(
                        self.tokens[self.current].clone(),
                    ));
                    return Err(ParserStatus::Unwind);
                }
            },
        };

        self.current += 1;
        let mut current_arg = 0;

        let mut elements = vec![];

        match self.current_token_type() {
            Some(TokenType::RightBracket) => {
                self.current += 1;
            }
            _ => loop {
                let expr = self.expression()?;

                let expr_value_type = self.expr_type(&expr);

                if expr_value_type != Some(value_type.clone()) {
                    self.error.add(ParserErrType::ArrayNonMatchingValue(
                        self.tokens[value_type_pos].clone(),
                        current_arg,
                        value_type,
                        expr_value_type,
                    ));
                    return Err(ParserStatus::Unwind);
                }

                elements.push(expr);

                current_arg += 1;

                match self.consume_token_type() {
                    Some(TokenType::Comma) => continue,
                    Some(TokenType::RightBracket) => break,
                    _ => {
                        self.error
                            .add(ParserErrType::ArrayExpectedCommaOrRightBracket(
                                self.tokens[self.current - 2].clone(),
                                self.tokens.get(self.current - 1).cloned(),
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };
            },
        }

        let class_name = Rc::new(format!("[{}]", print_type(&Some(value_type.clone()))));

        self.class_declarations.insert(
            Rc::clone(&class_name),
            array::new(class_name.clone(), value_type.clone()),
        );

        Ok(Expr::Object(
            Rc::clone(&class_name),
            ObjectExpr::NativeCall(match value_type {
                ValueType::Integer => NativeCallObject::IntegerArrayConstructor(elements),
                ValueType::Float => NativeCallObject::FloatArrayConstructor(elements),
                ValueType::String => NativeCallObject::StringArrayConstructor(elements),
                ValueType::Boolean => NativeCallObject::BooleanArrayConstructor(elements),
                ValueType::Class(_) => NativeCallObject::ObjectArrayConstructor(elements),
                _ => unreachable!(),
            }),
        ))
    }
}
