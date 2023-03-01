use crate::{
    assignment_operator::{
        BooleanAssignmentOperator, NumericAssignmentOperator, StringAssignmentOperator,
    },
    expr::{BooleanExpr, Expr, FloatExpr, IntegerExpr, StringExpr},
};

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    IntegerVariableDeclaration(IntegerExpr),
    FloatVariableDeclaration(FloatExpr),
    StringVariableDeclaration(StringExpr),
    BooleanVariableDeclaration(BooleanExpr),
    ClassVariableDeclaration(Vec<ConstructionType>),
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
