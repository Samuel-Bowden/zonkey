use crate::{
    comparison::{BooleanComparision, NumericComparision, StringComparision},
    native_function::{NativeFunctionNone, NativeFunctionString},
    operator::{NumericOperator, StringOperator},
    unary_operator::{BooleanUnaryOperator, NumericUnaryOperator},
};

#[derive(Debug)]
pub enum Expr {
    Integer(IntegerExpr),
    Float(FloatExpr),
    String(StringExpr),
    Boolean(BooleanExpr),
    Object(String, String, Vec<Expr>),
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
    Literal(String),
    Variable(usize),
    IntegerCast(Box<IntegerExpr>),
    FloatCast(Box<FloatExpr>),
    BooleanCast(Box<BooleanExpr>),
    NativeCall(NativeFunctionString),
    Call(usize, Vec<Expr>),
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
