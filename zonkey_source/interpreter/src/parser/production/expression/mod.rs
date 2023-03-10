mod addsub;
mod and;
mod call;
mod cast;
mod comparison;
mod equality;
mod literal;
mod multdiv;
mod or;
mod prelude;
mod unary;

use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn expression(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("expression");
        self.cast()
    }
}