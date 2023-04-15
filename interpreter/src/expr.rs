use crate::{standard_prelude::calls::*, parser::declaration::ConstructionType, token::Token};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(IntegerExpr),
    Float(FloatExpr),
    String(StringExpr),
    Boolean(BooleanExpr),
    Object(Rc<String>, ObjectExpr),
    None(NoneExpr),
}

impl Expr {
    pub fn to_string_expr(self) -> StringExpr {
        if let Expr::String(expr) = self {
            expr
        } else {
            panic!("Not a string expression")
        }
    }

    pub fn to_float_expr(self) -> FloatExpr {
        if let Expr::Float(expr) = self {
            expr
        } else {
            panic!("Not a float expression")
        }
    }

    pub fn to_integer_expr(self) -> IntegerExpr {
        if let Expr::Integer(expr) = self {
            expr
        } else {
            panic!("Not an integer expression")
        }
    }

    pub fn to_boolean_expr(self) -> BooleanExpr {
        if let Expr::Boolean(expr) = self {
            expr
        } else {
            panic!("Not a boolean expression")
        }
    }

    pub fn to_integer_expr_ref(&self) -> &IntegerExpr {
        if let Expr::Integer(expr) = self {
            expr
        } else {
            panic!("Not an integer expression")
        }
    }

    pub fn to_float_expr_ref(&self) -> &FloatExpr {
        if let Expr::Float(expr) = self {
            expr
        } else {
            panic!("Not a float expression")
        }
    }

    pub fn to_string_expr_ref(&self) -> &StringExpr {
        if let Expr::String(expr) = self {
            expr
        } else {
            panic!("Not a string expression")
        }
    }

    pub fn to_boolean_expr_ref(&self) -> &BooleanExpr {
        if let Expr::Boolean(expr) = self {
            expr
        } else {
            panic!("Not a boolean expression")
        }
    }

    pub fn to_object_expr(self) -> ObjectExpr {
        if let Expr::Object(_, expr) = self {
            expr
        } else {
            panic!("Not an object expression")
        }
    }

    pub fn to_object_expr_ref(&self) -> &ObjectExpr {
        if let Expr::Object(_, expr) = self {
            expr
        } else {
            panic!("Not an object expression")
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntegerExpr {
    Binary {
        left: Box<IntegerExpr>,
        operator: NumericOperator,
        right: Box<IntegerExpr>,
    },
    Literal(i64),
    Variable(usize),
    Property(usize, usize), // Object address, property address
    Unary(NumericUnaryOperator, Box<IntegerExpr>),
    Call(usize, Vec<Expr>), // Call address, argument expressions
    NativeCall(NativeCallInteger),
}

#[derive(Debug, Clone)]
pub enum FloatExpr {
    Binary {
        left: Box<FloatExpr>,
        operator: NumericOperator,
        right: Box<FloatExpr>,
    },
    Literal(f64),
    Variable(usize),
    Property(usize, usize), // Object address, property address
    Unary(NumericUnaryOperator, Box<FloatExpr>),
    Call(usize, Vec<Expr>), // Call address, argument expressions
    NativeCall(NativeCallFloat),
}

#[derive(Debug, Clone)]
pub enum StringExpr {
    Binary {
        left: Box<StringExpr>,
        operator: StringOperator,
        right: Box<StringExpr>,
    },
    Literal(Rc<String>),
    Variable(usize),
    Property(usize, usize), // Object address, property address
    Call(usize, Vec<Expr>), // Call address, argument expressions
    NativeCall(NativeCallString),
}

#[derive(Debug, Clone)]
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
    Property(usize, usize), // Object address, property address
    Unary(BooleanUnaryOperator, Box<BooleanExpr>),
    NativeCall(NativeCallBoolean),
    Call(usize, Vec<Expr>), // Call address, argument expressions
}

#[derive(Debug, Clone)]
pub enum NoneExpr {
    NativeCall(NativeCallNone),
    Call(usize, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum ObjectExpr {
    Variable(usize),
    Property(usize, usize), // Object address, property address
    Call(usize, Vec<Expr>), // Call name, Call address, argument expressions
    Constructor(Rc<Vec<ConstructionType>>),
    NativeCall(NativeCallObject),
}

#[derive(Debug, Clone)]
pub enum NumericOperator {
    Add,
    Subtract,
    Divide(Token),
    Multiply,
}

#[derive(Debug, Clone)]
pub enum StringOperator {
    Add,
}

#[derive(Debug, Clone)]
pub enum NumericUnaryOperator {
    Minus,
}

#[derive(Debug, Clone)]
pub enum BooleanUnaryOperator {
    Bang,
}

#[derive(Debug, Clone)]
pub enum NumericAssignmentOperator {
    Equal,
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,
}

#[derive(Debug, Clone)]
pub enum StringAssignmentOperator {
    Equal,
    PlusEqual,
}

#[derive(Debug, Clone)]
pub enum BooleanAssignmentOperator {
    Equal,
}

#[derive(Debug, Clone)]
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
