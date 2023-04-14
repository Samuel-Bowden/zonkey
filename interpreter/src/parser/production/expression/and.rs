use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn and(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("and");

        let mut left = self.equality()?;

        loop {
            if let Some(TokenType::And) = self.current_token_type() {
                let and_token_pos = self.current;
                self.current += 1;

                let right = self.equality()?;

                match (left, right) {
                    (Expr::Boolean(left_inside), Expr::Boolean(right_inside)) => {
                        left = Expr::Boolean(BooleanExpr::BooleanBinary {
                            left: Box::new(left_inside),
                            comparator: BooleanComparision::And,
                            right: Box::new(right_inside),
                        })
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        if left == right {
                            self.error.add(ParserErrType::ComparisionInvalidForType(
                                self.tokens[and_token_pos].clone(),
                                left,
                            ));
                        } else {
                            self.error.add(ParserErrType::ComparisionUnmatchingTypes(
                                self.tokens[and_token_pos].clone(),
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
