use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn unary(&mut self) -> Result<Expr, ParserStatus> {
        if let Some(TokenType::Minus | TokenType::Bang) = self.current_token_type() {
            let operator_pos = self.current;
            self.current += 1;

            let unary_expr = self.unary()?;

            let operator_type = &self.tokens[operator_pos].token_type;

            match (operator_type, unary_expr) {
                (TokenType::Minus, Expr::Integer(expr)) => Ok(Expr::Integer(IntegerExpr::Unary(
                    NumericUnaryOperator::Minus,
                    Box::new(expr),
                ))),
                (TokenType::Minus, Expr::Float(expr)) => Ok(Expr::Float(FloatExpr::Unary(
                    NumericUnaryOperator::Minus,
                    Box::new(expr),
                ))),
                (TokenType::Bang, Expr::Boolean(expr)) => Ok(Expr::Boolean(BooleanExpr::Unary(
                    BooleanUnaryOperator::Bang,
                    Box::new(expr),
                ))),
                (_, expr) => {
                    let expr_type = self.expr_type(&expr);

                    self.error.add(ParserErrType::UnaryOperatorInvalidForType(
                        self.tokens[operator_pos].clone(),
                        expr_type,
                    ));

                    Err(ParserStatus::Unwind)
                }
            }
        } else {
            self.value()
        }
    }
}
