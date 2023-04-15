use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn multdiv(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("multdiv");

        let mut left = self.unary()?;

        loop {
            if let Some(TokenType::Star | TokenType::Slash) = self.current_token_type() {
                let operator_token_pos = self.current;
                self.current += 1;

                let right = self.unary()?;

                let operator_type = &self.tokens[operator_token_pos].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Integer(IntegerExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Star => NumericOperator::Multiply,
                                _ => {
                                    NumericOperator::Divide(self.tokens[operator_token_pos].clone())
                                }
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Float(FloatExpr::Binary {
                            left: Box::new(left_inside),
                            operator: match operator_type {
                                TokenType::Star => NumericOperator::Multiply,
                                _ => {
                                    NumericOperator::Divide(self.tokens[operator_token_pos].clone())
                                }
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::OperatorInvalidForType(
                                self.tokens[operator_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::OperatorUnmatchingTypes(
                                self.tokens[operator_token_pos].clone(),
                                left,
                                right,
                            ));
                        }

                        return Err(ParserStatus::Unwind);
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }
}
