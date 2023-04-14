use crate::parser::production::statement::prelude::*;

impl Parser {
    pub fn while_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("while_statement");

        let left_paren = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::WhileExpectedLeftParen(
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
                self.error.add(ParserErrType::WhileExpectedRightParen(
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
            self.error.add(ParserErrType::WhileConditionNotBool(
                left_paren + 1,
                right_paren - 1,
            ));
            // Place dummy expression to continue parsing rest for errors
            BooleanExpr::Literal(false)
        };

        self.loop_count += 1;
        let block = Box::new(self.block()?);
        self.returned_value = false;
        self.loop_count -= 1;

        Ok(Stmt::While(expression, block))
    }
}
