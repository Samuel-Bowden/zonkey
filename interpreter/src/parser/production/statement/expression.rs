use std::rc::Rc;

use crate::{expr::*, parser::production::statement::prelude::*, parser::value::ValueType};

impl Parser {
    fn to_numeric_asgmt_op(&self, pos: usize) -> NumericAssignmentOperator {
        match self.tokens[pos].token_type {
            TokenType::Equal => NumericAssignmentOperator::Equal,
            TokenType::PlusEqual => NumericAssignmentOperator::PlusEqual,
            TokenType::MinusEqual => NumericAssignmentOperator::MinusEqual,
            TokenType::StarEqual => NumericAssignmentOperator::StarEqual,
            _ => NumericAssignmentOperator::SlashEqual,
        }
    }

    fn to_string_asgmt_op(&mut self, pos: usize) -> Result<StringAssignmentOperator, ParserStatus> {
        match self.tokens[pos].token_type {
            TokenType::Equal => Ok(StringAssignmentOperator::Equal),
            TokenType::PlusEqual => Ok(StringAssignmentOperator::PlusEqual),
            _ => {
                self.error.add(ParserErrType::InvalidAssignmentOperator(
                    self.tokens[pos].clone(),
                    ValueType::String,
                ));
                Err(ParserStatus::Unwind)
            }
        }
    }

    fn to_boolean_asgmt_op(
        &mut self,
        pos: usize,
    ) -> Result<BooleanAssignmentOperator, ParserStatus> {
        match self.tokens[pos].token_type {
            TokenType::Equal => Ok(BooleanAssignmentOperator::Equal),
            _ => {
                self.error.add(ParserErrType::InvalidAssignmentOperator(
                    self.tokens[pos].clone(),
                    ValueType::Boolean,
                ));
                Err(ParserStatus::Unwind)
            }
        }
    }

    fn to_obj_asgmt_op(
        &mut self,
        pos: usize,
        variable_class: Rc<String>,
    ) -> Result<ObjectAssignmentOperator, ParserStatus> {
        match self.tokens[pos].token_type {
            TokenType::Equal => Ok(ObjectAssignmentOperator::Equal),
            _ => {
                self.error.add(ParserErrType::InvalidAssignmentOperator(
                    self.tokens[pos].clone(),
                    ValueType::Class(Rc::clone(&variable_class)),
                ));
                Err(ParserStatus::Unwind)
            }
        }
    }

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
                            self.to_numeric_asgmt_op(assignment_operator),
                        ))
                    }
                    (Expr::Integer(IntegerExpr::Property(obj_id, id)), Expr::Integer(val)) => {
                        Ok(Stmt::IntegerPropertyAssignment(
                            obj_id,
                            id,
                            val,
                            self.to_numeric_asgmt_op(assignment_operator),
                        ))
                    }
                    (Expr::Float(FloatExpr::Variable(id)), Expr::Float(val)) => {
                        Ok(Stmt::FloatVariableAssignment(
                            id,
                            val,
                            self.to_numeric_asgmt_op(assignment_operator),
                        ))
                    }
                    (Expr::Float(FloatExpr::Property(obj_id, id)), Expr::Float(val)) => {
                        Ok(Stmt::FloatPropertyAssignment(
                            obj_id,
                            id,
                            val,
                            self.to_numeric_asgmt_op(assignment_operator),
                        ))
                    }
                    (Expr::String(StringExpr::Variable(id)), Expr::String(val)) => {
                        Ok(Stmt::StringVariableAssignment(
                            id,
                            val,
                            self.to_string_asgmt_op(assignment_operator)?,
                        ))
                    }
                    (Expr::String(StringExpr::Property(obj_id, id)), Expr::String(val)) => {
                        Ok(Stmt::StringPropertyAssignment(
                            obj_id,
                            id,
                            val,
                            self.to_string_asgmt_op(assignment_operator)?,
                        ))
                    }
                    (Expr::Boolean(BooleanExpr::Variable(id)), Expr::Boolean(val)) => {
                        Ok(Stmt::BooleanVariableAssignment(
                            id,
                            val,
                            self.to_boolean_asgmt_op(assignment_operator)?,
                        ))
                    }
                    (Expr::Boolean(BooleanExpr::Property(obj_id, id)), Expr::Boolean(val)) => {
                        Ok(Stmt::BooleanPropertyAssignment(
                            obj_id,
                            id,
                            val,
                            self.to_boolean_asgmt_op(assignment_operator)?,
                        ))
                    }
                    (
                        Expr::Object(variable_class, ObjectExpr::Variable(id)),
                        Expr::Object(expr_class, val),
                    ) if variable_class == expr_class => Ok(Stmt::ObjectVariableAssignment(
                        id,
                        val,
                        self.to_obj_asgmt_op(assignment_operator, variable_class)?,
                    )),
                    (
                        Expr::Object(variable_class, ObjectExpr::Property(obj_id, id)),
                        Expr::Object(expr_class, val),
                    ) if variable_class == expr_class => Ok(Stmt::ObjectPropertyAssignment(
                        obj_id,
                        id,
                        val,
                        self.to_obj_asgmt_op(assignment_operator, variable_class)?,
                    )),
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
