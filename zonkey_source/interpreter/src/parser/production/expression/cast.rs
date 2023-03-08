use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn cast(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("cast");

        match self.current_token_type() {
            Some(TokenType::IntegerType) => {
                let integer_type_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match expression {
                    Expr::Integer(expr) => {
                        self.error.add(ParserErrType::CastPointless(
                            self.tokens[integer_type_pos].clone(),
                            Some(ValueType::Integer),
                        ));

                        Ok(Expr::Integer(expr))
                    }
                    Expr::Float(expr) => Ok(Expr::Integer(IntegerExpr::FloatCast(Box::new(expr)))),
                    Expr::Boolean(expr) => {
                        Ok(Expr::Integer(IntegerExpr::BooleanCast(Box::new(expr))))
                    }
                    Expr::String(expr) => {
                        Ok(Expr::Integer(IntegerExpr::StringCast(Box::new(expr))))
                    }
                    expr => {
                        self.error.add(ParserErrType::CastNotPossible(
                            self.tokens[integer_type_pos].clone(),
                            Some(ValueType::Integer),
                            self.expr_type(&expr),
                        ));

                        Err(ParserStatus::Unwind)
                    }
                }
            }
            Some(TokenType::FloatType) => {
                let float_type_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match expression {
                    Expr::Float(expr) => {
                        self.error.add(ParserErrType::CastPointless(
                            self.tokens[float_type_pos].clone(),
                            Some(ValueType::Float),
                        ));

                        Ok(Expr::Float(expr))
                    }
                    Expr::Integer(expr) => Ok(Expr::Float(FloatExpr::IntegerCast(Box::new(expr)))),
                    Expr::Boolean(expr) => Ok(Expr::Float(FloatExpr::BooleanCast(Box::new(expr)))),
                    Expr::String(expr) => Ok(Expr::Float(FloatExpr::StringCast(Box::new(expr)))),
                    expr => {
                        self.error.add(ParserErrType::CastNotPossible(
                            self.tokens[float_type_pos].clone(),
                            Some(ValueType::Float),
                            self.expr_type(&expr),
                        ));

                        Err(ParserStatus::Unwind)
                    }
                }
            }
            Some(TokenType::StringType) => {
                let string_type_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match expression {
                    Expr::String(expr) => {
                        self.error.add(ParserErrType::CastPointless(
                            self.tokens[string_type_pos].clone(),
                            Some(ValueType::String),
                        ));

                        Ok(Expr::String(expr))
                    }
                    Expr::Integer(expr) => {
                        Ok(Expr::String(StringExpr::IntegerCast(Box::new(expr))))
                    }
                    Expr::Float(expr) => Ok(Expr::String(StringExpr::FloatCast(Box::new(expr)))),
                    Expr::Boolean(expr) => {
                        Ok(Expr::String(StringExpr::BooleanCast(Box::new(expr))))
                    }
                    expr => {
                        self.error.add(ParserErrType::CastNotPossible(
                            self.tokens[string_type_pos].clone(),
                            Some(ValueType::String),
                            self.expr_type(&expr),
                        ));

                        Err(ParserStatus::Unwind)
                    }
                }
            }
            Some(TokenType::BooleanType) => {
                let boolean_type_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match expression {
                    Expr::Boolean(expr) => {
                        self.error.add(ParserErrType::CastPointless(
                            self.tokens[boolean_type_pos].clone(),
                            Some(ValueType::Boolean),
                        ));

                        Ok(Expr::Boolean(expr))
                    }
                    Expr::Integer(expr) => {
                        Ok(Expr::Boolean(BooleanExpr::IntegerCast(Box::new(expr))))
                    }
                    Expr::Float(expr) => Ok(Expr::Boolean(BooleanExpr::FloatCast(Box::new(expr)))),
                    Expr::String(expr) => {
                        Ok(Expr::Boolean(BooleanExpr::StringCast(Box::new(expr))))
                    }
                    expr => {
                        self.error.add(ParserErrType::CastNotPossible(
                            self.tokens[boolean_type_pos].clone(),
                            Some(ValueType::Boolean),
                            self.expr_type(&expr),
                        ));

                        Err(ParserStatus::Unwind)
                    }
                }
            }
            _ => self.or(),
        }
    }
}
