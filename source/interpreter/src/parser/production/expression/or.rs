use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn or(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("or");

        let mut left = self.and()?;

        loop {
            if let Some(TokenType::Or) = self.current_token_type() {
                let or_token_pos = self.current;
                self.current += 1;

                let right = self.and()?;

                match (left, right) {
                    (Expr::Boolean(left_inside), Expr::Boolean(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::BooleanBinary {
                            left: Box::new(left_inside),
                            comparator: BooleanComparision::Or,
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::ComparisionInvalidForType(
                                self.tokens[or_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                                self.tokens[or_token_pos].clone(),
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
