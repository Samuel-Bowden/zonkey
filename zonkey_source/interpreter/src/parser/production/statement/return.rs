use crate::parser::{production::statement::prelude::*, value::ValueType};

impl Parser {
    pub fn return_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("return_statement");

        let return_token_position = self.current;
        self.current += 1;

        let expression = match self.current_token_type() {
            Some(TokenType::SemiColon) => None,
            _ => Some(self.expression()?),
        };

        Ok(Stmt::Return(
            match (&self.current_return_type, expression) {
                (Some(ValueType::Integer), Some(Expr::Integer(expr))) => Some(Expr::Integer(expr)),
                (Some(ValueType::Float), Some(Expr::Float(expr))) => Some(Expr::Float(expr)),
                (Some(ValueType::String), Some(Expr::String(expr))) => Some(Expr::String(expr)),
                (Some(ValueType::Boolean), Some(Expr::Boolean(expr))) => Some(Expr::Boolean(expr)),
                (None, Some(Expr::None(expr))) => Some(Expr::None(expr)),
                (None, None) => None,
                (ret_type, expr) => {
                    let expr_type = if let Some(expr) = expr {
                        self.expr_type(&expr)
                    } else {
                        None
                    };

                    self.error.add(
                        ParserErrType::FunctionDeclarationInvalidReturnExpressionType(
                            self.tokens[return_token_position].clone(),
                            ret_type.clone(),
                            expr_type,
                        ),
                    );

                    return Err(ParserStatus::Unwind);
                }
            },
        ))
    }
}
