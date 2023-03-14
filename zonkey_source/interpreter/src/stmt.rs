use crate::{expr::*, stack::Stack};

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    IntegerVariableInitialisation(IntegerExpr),
    FloatVariableInitialisation(FloatExpr),
    StringVariableInitialisation(StringExpr),
    BooleanVariableInitialisation(BooleanExpr),
    DefaultConstructor(Vec<ConstructionType>),
    ObjectVariableInitialisation(ObjectExpr),
    IntegerVariableAssignment(usize, IntegerExpr, NumericAssignmentOperator),
    FloatVariableAssignment(usize, FloatExpr, NumericAssignmentOperator),
    StringVariableAssignment(usize, StringExpr, StringAssignmentOperator),
    BooleanVariableAssignment(usize, BooleanExpr, BooleanAssignmentOperator),
    IntegerPropertyAssignment(Vec<usize>, usize, IntegerExpr, NumericAssignmentOperator),
    FloatPropertyAssignment(Vec<usize>, usize, FloatExpr, NumericAssignmentOperator),
    StringPropertyAssignment(Vec<usize>, usize, StringExpr, StringAssignmentOperator),
    BooleanPropertyAssignment(Vec<usize>, usize, BooleanExpr, BooleanAssignmentOperator),
    ObjectPropertyAssignment(Vec<usize>, usize, ObjectExpr, ObjectAssignmentOperator),
    Block(Vec<Stmt>, Stack),
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
