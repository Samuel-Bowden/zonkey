use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn grouping(&mut self) -> Result<Expr, ParserStatus> {
        let left_paren_pos = self.current - 1;

        self.sub_expression_limit += 1;

        if self.sub_expression_limit > 50 {
            self.error.add(ParserErrType::SubExpressionLimit(
                self.tokens[left_paren_pos].clone(),
            ));
            return Err(ParserStatus::Unwind);
        }

        let expression = self.expression()?;

        self.sub_expression_limit -= 1;

        match self.consume_token_type() {
            Some(TokenType::RightParen) => Ok(expression),
            _ => {
                self.error.add(ParserErrType::GroupingExpectedRightParen(
                    self.tokens[left_paren_pos].clone(),
                    self.tokens.get(self.current - 1).cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        }
    }
}
