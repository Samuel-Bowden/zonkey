use crate::{expr::*, stack::Stack};

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    IntegerVariableInitialisation(IntegerExpr),
    FloatVariableInitialisation(FloatExpr),
    StringVariableInitialisation(StringExpr),
    BooleanVariableInitialisation(BooleanExpr),
    ObjectVariableInitialisation(ObjectExpr),
    IntegerVariableAssignment(usize, IntegerExpr, NumericAssignmentOperator),
    FloatVariableAssignment(usize, FloatExpr, NumericAssignmentOperator),
    StringVariableAssignment(usize, StringExpr, StringAssignmentOperator),
    BooleanVariableAssignment(usize, BooleanExpr, BooleanAssignmentOperator),
    ObjectVariableAssignment(usize, ObjectExpr, ObjectAssignmentOperator),
    IntegerPropertyAssignment(usize, usize, IntegerExpr, NumericAssignmentOperator),
    FloatPropertyAssignment(usize, usize, FloatExpr, NumericAssignmentOperator),
    StringPropertyAssignment(usize, usize, StringExpr, StringAssignmentOperator),
    BooleanPropertyAssignment(usize, usize, BooleanExpr, BooleanAssignmentOperator),
    ObjectPropertyAssignment(usize, usize, ObjectExpr, ObjectAssignmentOperator),
    Block(Vec<Stmt>, Stack),
    If(BooleanExpr, Box<Stmt>, Option<Box<Stmt>>),
    While(BooleanExpr, Box<Stmt>),
    Loop(Box<Stmt>),
    Break,
    Continue,
    Return(Option<Expr>),
}
