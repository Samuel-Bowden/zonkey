use crate::parser::production::statement::prelude::*;

impl Parser {
    pub fn if_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("if_statement");

        let left_paren = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::IfExpectedLeftParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let expression = self.expression()?;

        let right_paren = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::RightParen,
                end,
                ..
            }) => *end,
            t => {
                self.error.add(ParserErrType::IfExpectedRightParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };

        self.current += 1;

        let expression = if let Expr::Boolean(expr) = expression {
            expr
        } else {
            self.error.add(ParserErrType::IfConditionNotBool(
                left_paren + 1,
                right_paren - 1,
            ));
            // Place dummy expression to continue parsing rest for errors
            BooleanExpr::Literal(false)
        };

        let true_branch = Box::new(self.block()?);

        let false_branch = match self.current_token_type() {
            Some(TokenType::Else) => {
                self.current += 1;

                Some(Box::new(self.statement()?))
            }
            _ => None,
        };

        Ok(Stmt::If(expression, true_branch, false_branch))
    }
}
