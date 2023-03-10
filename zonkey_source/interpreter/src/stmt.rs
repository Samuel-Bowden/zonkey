use crate::expr::*;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    IntegerVariableInitialisation(IntegerExpr),
    FloatVariableInitialisation(FloatExpr),
    StringVariableInitialisation(StringExpr),
    BooleanVariableInitialisation(BooleanExpr),
    ClassVariableInitialisation(Vec<ConstructionType>),
    IntegerVariableAssignment(usize, IntegerExpr, NumericAssignmentOperator),
    FloatVariableAssignment(usize, FloatExpr, NumericAssignmentOperator),
    StringVariableAssignment(usize, StringExpr, StringAssignmentOperator),
    BooleanVariableAssignment(usize, BooleanExpr, BooleanAssignmentOperator),
    Block(Vec<Stmt>, (usize, usize, usize, usize)),
    If(BooleanExpr, Box<Stmt>, Option<Box<Stmt>>),
    While(BooleanExpr, Box<Stmt>),
    Loop(Box<Stmt>),
    Break,
    Continue,
    Return(Option<Expr>),
}

#[derive(Debug)]
pub enum ConstructionType {
    Integer,
    Float,
    String,
    Boolean,
    Class(Vec<ConstructionType>),
}
