use crate::parser::production::statement::prelude::*;
use indexmap::IndexMap;

impl Parser {
    pub fn for_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("for_statement");

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftParen,
                ..
            }) => {
                self.current += 1;
            }
            t => {
                self.error.add(ParserErrType::ForExpectedLeftParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };

        self.value_stack.push(IndexMap::new());
        let integer_point = self.integer_next_id;
        let float_point = self.float_next_id;
        let string_point = self.string_next_id;
        let boolean_point = self.boolean_next_id;

        // Abort parsing when there are errors parsing the parameters, as a block has been
        // added and it will be very difficult to synchronise.
        let initialiser_statement = match self.variable_init() {
            Ok(is) => is,
            Err(_) => return Err(ParserStatus::End),
        };

        let test_statement_start = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Comma,
                end,
                ..
            }) => *end,
            t => {
                self.error.add(ParserErrType::ForExpectedComma1(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };
        self.current += 1;

        let test_statement = match self.expression() {
            Ok(ts) => ts,
            Err(_) => return Err(ParserStatus::End),
        };

        let test_statement_end = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Comma,
                start,
                ..
            }) => *start,
            t => {
                self.error.add(ParserErrType::ForExpectedComma2(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };
        self.current += 1;

        let test_statement = if let Expr::Boolean(expr) = test_statement {
            expr
        } else {
            self.error.add(ParserErrType::ForConditionNotBool(
                test_statement_start,
                test_statement_end,
            ));
            // Place dummy expression to continue parsing rest for errors
            BooleanExpr::Literal(false)
        };

        let update_statement = match self.expression_statement() {
            Ok(us) => us,
            Err(_) => return Err(ParserStatus::End),
        };

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::RightParen,
                ..
            }) => {
                self.current += 1;
            }
            t => {
                self.error.add(ParserErrType::ForExpectedRightParen(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::End);
            }
        };

        let mut block = self.block()?;

        if let Stmt::Block(b, _) = &mut block {
            b.push(update_statement);
        }

        self.value_stack.pop();

        self.integer_next_id = integer_point;
        self.float_next_id = float_point;
        self.string_next_id = string_point;
        self.boolean_next_id = boolean_point;

        Ok(Stmt::Block(
            vec![
                initialiser_statement,
                Stmt::While(test_statement, Box::new(block)),
            ],
            (integer_point, float_point, string_point, boolean_point),
        ))
    }
}
