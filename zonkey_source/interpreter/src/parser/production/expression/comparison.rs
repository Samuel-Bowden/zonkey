use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn comparision(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("comparison");

        let mut left = self.addsub()?;

        loop {
            if let Some(
                TokenType::MoreEqual | TokenType::LessEqual | TokenType::Less | TokenType::More,
            ) = self.current_token_type()
            {
                let comparator_token_pos = self.current;
                self.current += 1;

                let right = self.addsub()?;

                let comparator_type = &self.tokens[comparator_token_pos].token_type;

                match (left, right) {
                    (Expr::Integer(left_inside), Expr::Integer(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::IntegerBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::MoreEqual => NumericComparision::MoreEqual,
                                TokenType::LessEqual => NumericComparision::LessEqual,
                                TokenType::More => NumericComparision::More,
                                _ => NumericComparision::Less,
                            },
                            right: Box::new(right_inside),
                        })
                    }
                    (Expr::Float(left_inside), Expr::Float(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::FloatBinary {
                            left: Box::new(left_inside),
                            comparator: match comparator_type {
                                TokenType::MoreEqual => NumericComparision::MoreEqual,
                                TokenType::LessEqual => NumericComparision::LessEqual,
                                TokenType::More => NumericComparision::More,
                                _ => NumericComparision::Less,
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
