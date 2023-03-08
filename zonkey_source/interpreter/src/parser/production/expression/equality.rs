use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn equality(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("equality");

        let mut left = self.comparision()?;

        loop {
            if let Some(TokenType::EqualEqual | TokenType::BangEqual) = self.current_token_type() {
                let comparator_token_pos = self.current;
                self.current += 1;

                let right = self.comparision()?;

                let comparator_type = &self.tokens[comparator_token_pos].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::IntegerBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => NumericComparision::Equal,
                                _ => NumericComparision::Inequal,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::FloatBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => NumericComparision::Equal,
                                _ => NumericComparision::Inequal,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::String(left_inside), Expr::String(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::StringBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => StringComparision::Equal,
                                _ => StringComparision::Inequal,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Boolean(left_inside), Expr::Boolean(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::BooleanBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::EqualEqual => BooleanComparision::Equal,
                                _ => BooleanComparision::Inequal,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::ComparisionInvalidForType(
                                self.tokens[comparator_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                                self.tokens[comparator_token_pos].clone(),
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
