use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn unary(&mut self) -> Result<Expr, ParserStatus> {
        // Parse all unary operators
        let mut tokens = vec![];

        while let Some(TokenType::Minus | TokenType::Bang) = self.current_token_type() {
            tokens.push(self.current);
            self.current += 1;
        }

        // Get the value expression
        let mut expression = self.value()?;

        for operator_pos in tokens.iter().rev() {
            let operator_type = &self.tokens[*operator_pos].token_type;

            expression = match (operator_type, expression) {
                (TokenType::Minus, Expr::Integer(expr)) => Expr::Integer(IntegerExpr::Unary(
                    NumericUnaryOperator::Minus,
                    Box::new(expr),
                )),
                (TokenType::Minus, Expr::Float(expr)) => Expr::Float(FloatExpr::Unary(
                    NumericUnaryOperator::Minus,
                    Box::new(expr),
                )),
                (TokenType::Bang, Expr::Boolean(expr)) => Expr::Boolean(BooleanExpr::Unary(
                    BooleanUnaryOperator::Bang,
                    Box::new(expr),
                )),
                (_, expr) => {
                    let expr_type = self.expr_type(&expr);

                    self.error.add(ParserErrType::UnaryOperatorInvalidForType(
                        self.tokens[*operator_pos].clone(),
                        expr_type,
                    ));

                    return Err(ParserStatus::Unwind);
                }
            }
        }

        Ok(expression)
    }
}
