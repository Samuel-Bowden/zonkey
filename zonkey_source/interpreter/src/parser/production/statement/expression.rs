use crate::{expr::*, parser::production::statement::prelude::*, parser::value::ValueType};

impl Parser {
    pub fn expression_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("expression_statement");

        let expr = self.expression()?;

        match self.current_token_type() {
            Some(
                TokenType::Equal
                | TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual,
            ) => {
                let assignment_operator = self.current;
                self.current += 1;

                let value = self.expression()?;

                match (expr, value) {
                    (Expr::Integer(IntegerExpr::Variable(id)), Expr::Integer(val)) => {
                        Ok(Stmt::IntegerVariableAssignment(
                            id,
                            val,
                            match self.tokens[assignment_operator].token_type {
                                TokenType::Equal => NumericAssignmentOperator::Equal,
                                TokenType::PlusEqual => NumericAssignmentOperator::PlusEqual,
                                TokenType::MinusEqual => NumericAssignmentOperator::MinusEqual,
                                TokenType::StarEqual => NumericAssignmentOperator::StarEqual,
                                _ => NumericAssignmentOperator::SlashEqual,
                            },
                        ))
                    }
                    (Expr::Float(FloatExpr::Variable(id)), Expr::Float(val)) => {
                        Ok(Stmt::FloatVariableAssignment(
                            id,
                            val,
                            match self.tokens[assignment_operator].token_type {
                                TokenType::Equal => NumericAssignmentOperator::Equal,
                                TokenType::PlusEqual => NumericAssignmentOperator::PlusEqual,
                                TokenType::MinusEqual => NumericAssignmentOperator::MinusEqual,
                                TokenType::StarEqual => NumericAssignmentOperator::StarEqual,
                                _ => NumericAssignmentOperator::SlashEqual,
                            },
                        ))
                    }
                    (Expr::String(StringExpr::Variable(id)), Expr::String(val)) => {
                        Ok(Stmt::StringVariableAssignment(
                            id,
                            val,
                            match self.tokens[assignment_operator].token_type {
                                TokenType::Equal => StringAssignmentOperator::Equal,
                                TokenType::PlusEqual => StringAssignmentOperator::PlusEqual,
                                _ => {
                                    self.error.add(ParserErrType::InvalidAssignmentOperator(
                                        self.tokens[assignment_operator].clone(),
                                        ValueType::String,
                                    ));
                                    return Err(ParserStatus::Unwind);
                                }
                            },
                        ))
                    }
                    (Expr::Boolean(BooleanExpr::Variable(id)), Expr::Boolean(val)) => {
                        Ok(Stmt::BooleanVariableAssignment(
                            id,
                            val,
                            match self.tokens[assignment_operator].token_type {
                                TokenType::Equal => BooleanAssignmentOperator::Equal,
                                _ => {
                                    self.error.add(ParserErrType::InvalidAssignmentOperator(
                                        self.tokens[assignment_operator].clone(),
                                        ValueType::Boolean,
                                    ));
                                    return Err(ParserStatus::Unwind);
                                }
                            },
                        ))
                    }
                    (left, right) => {
                        let left = self.expr_type(&left);
                        let right = self.expr_type(&right);

                        self.error
                            .add(ParserErrType::UnmatchingTypesAssignmentOperatator(
                                self.tokens[assignment_operator].clone(),
                                left,
                                right,
                            ));

                        return Err(ParserStatus::Unwind);
                    }
                }
            }
            _ => Ok(Stmt::Expression(expr)),
        }
    }
}
