mod addsub;
mod and;
mod comparison;
mod equality;
mod function_call;
mod grouping;
mod literal;
mod method_call;
mod multdiv;
mod or;
mod prelude;
mod unary;
mod array_constructor;

use crate::parser::production::expression::prelude::*;

impl Parser {
    pub fn expression(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("expression");
        self.or()
    }
}
