use crate::prelude::*;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Integer(IntegerExpr),
    Float(FloatExpr),
    String(StringExpr),
    Boolean(BooleanExpr),
    Object(Rc<String>, ObjectExpr),
    None(NoneExpr),
}

#[derive(Debug)]
pub enum IntegerExpr {
    Binary {
        left: Box<IntegerExpr>,
        operator: NumericOperator,
        right: Box<IntegerExpr>,
    },
    Literal(i64),
    Variable(usize),
    Property(Vec<usize>, usize),
    FloatCast(Box<FloatExpr>),
    BooleanCast(Box<BooleanExpr>),
    StringCast(Box<StringExpr>),
    Unary(NumericUnaryOperator, Box<IntegerExpr>),
    Call(usize, Vec<Expr>),
}

#[derive(Debug)]
pub enum FloatExpr {
    Binary {
        left: Box<FloatExpr>,
        operator: NumericOperator,
        right: Box<FloatExpr>,
    },
    Literal(f64),
    Variable(usize),
    Property(Vec<usize>, usize),
    IntegerCast(Box<IntegerExpr>),
    BooleanCast(Box<BooleanExpr>),
    StringCast(Box<StringExpr>),
    Unary(NumericUnaryOperator, Box<FloatExpr>),
    Call(usize, Vec<Expr>),
}

#[derive(Debug)]
pub enum StringExpr {
    Binary {
        left: Box<StringExpr>,
        operator: StringOperator,
        right: Box<StringExpr>,
    },
    Literal(Rc<String>),
    Variable(usize),
    Property(Vec<usize>, usize),
    IntegerCast(Box<IntegerExpr>),
    FloatCast(Box<FloatExpr>),
    BooleanCast(Box<BooleanExpr>),
    Call(usize, Vec<Expr>),
    NativeCall(NativeFunctionString),
}

#[derive(Debug)]
pub enum BooleanExpr {
    IntegerBinary {
        left: Box<IntegerExpr>,
        comparator: NumericComparision,
        right: Box<IntegerExpr>,
    },
    FloatBinary {
        left: Box<FloatExpr>,
        comparator: NumericComparision,
        right: Box<FloatExpr>,
    },
    StringBinary {
        left: Box<StringExpr>,
        comparator: StringComparision,
        right: Box<StringExpr>,
    },
    BooleanBinary {
        left: Box<BooleanExpr>,
        comparator: BooleanComparision,
        right: Box<BooleanExpr>,
    },
    Literal(bool),
    Variable(usize),
    Property(Vec<usize>, usize),
    IntegerCast(Box<IntegerExpr>),
    FloatCast(Box<FloatExpr>),
    StringCast(Box<StringExpr>),
    Unary(BooleanUnaryOperator, Box<BooleanExpr>),
    Call(usize, Vec<Expr>),
}

#[derive(Debug)]
pub enum NoneExpr {
    NativeCall(NativeFunctionNone),
    Call(usize, Vec<Expr>),
}

#[derive(Debug)]
pub enum ObjectExpr {
    Variable(usize),
    Property(Vec<usize>, usize),
    Call(usize, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum NumericOperator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug, Clone)]
pub enum StringOperator {
    Add,
}

#[derive(Debug)]
pub enum NumericUnaryOperator {
    Minus,
}

#[derive(Debug)]
pub enum BooleanUnaryOperator {
    Bang,
}

#[derive(Debug)]
pub enum NumericAssignmentOperator {
    Equal,
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,
}

#[derive(Debug)]
pub enum StringAssignmentOperator {
    Equal,
    PlusEqual,
}

#[derive(Debug)]
pub enum BooleanAssignmentOperator {
    Equal,
}

#[derive(Debug)]
pub enum ObjectAssignmentOperator {
    Equal,
}

#[derive(Debug, Clone)]
pub enum NumericComparision {
    Equal,
    Inequal,
    MoreEqual,
    More,
    LessEqual,
    Less,
}

#[derive(Debug, Clone)]
pub enum StringComparision {
    Equal,
    Inequal,
}

#[derive(Debug, Clone)]
pub enum BooleanComparision {
    Equal,
    Inequal,
    And,
    Or,
}
