use crate::parser::production::statement::prelude::*;

impl Parser {
    pub fn loop_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("loop_statement");

        self.loop_count += 1;
        let block = Box::new(self.block()?);
        self.returned_value = false;
        self.loop_count -= 1;

        Ok(Stmt::Loop(block))
    }
}
