use crate::parser::production::statement::prelude::*;

impl Parser {
    pub fn loop_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("loop_statement");

        let block = Box::new(self.block()?);

        Ok(Stmt::Loop(block))
    }
}
