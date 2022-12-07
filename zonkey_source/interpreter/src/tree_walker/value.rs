use std::{ops::{Sub, Add, Div, Mul}, fmt::Display};

use super::err::TreeWalkerErr;

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
}

impl Add for Value {
    type Output = Result<Self, TreeWalkerErr>;

    fn add(self, other: Self) -> Self::Output {
         match self {
            Self::Integer(left) => {
                match other {
                    Self::Integer(right) => Ok(Self::Integer(left + right)),
                    Self::Float(right) => Ok(Self::Float(left as f64 + right)),
                    Self::String(_) => Err(TreeWalkerErr::AddStringToInteger),
                }
            }
            Self::Float(left) => {
                match other {
                    Self::Integer(right) => Ok(Self::Float(left + right as f64)),
                    Self::Float(right) => Ok(Self::Float(left + right)),
                    Self::String(_) => Err(TreeWalkerErr::AddStringToFloat),
                }
            }
            Self::String(left) => {
                match other {
                    Self::Integer(_) => Err(TreeWalkerErr::AddIntegerToString),
                    Self::Float(_) => Err(TreeWalkerErr::AddFloatToString),
                    Self::String(right) => Ok(Self::String(left + &right)),
                }
            }
        }

    }
}

impl Sub for Value {
    type Output = Result<Self, TreeWalkerErr>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(left) => {
                match other {
                    Self::Integer(right) => Ok(Self::Integer(left - right)),
                    Self::Float(right) => Ok(Self::Float(left as f64 - right)),
                    Self::String(_) => Err(TreeWalkerErr::SubtractStringFromInteger),
                }
            }
            Self::Float(left) => {
                match other {
                    Self::Integer(right) => Ok(Self::Float(left - right as f64)),
                    Self::Float(right) => Ok(Self::Float(left - right)),
                    Self::String(_) => Err(TreeWalkerErr::SubtractStringFromFloat),
                }
            }
            Self::String(_) => {
                match other {
                    Self::Integer(_) => Err(TreeWalkerErr::SubtractIntegerFromString),
                    Self::Float(_) => Err(TreeWalkerErr::SubtractFloatFromString),
                    Self::String(_) => Err(TreeWalkerErr::SubtractStringFromString),
                }
            }
        }
    }
}

impl Div for Value {
    type Output = Result<Self, TreeWalkerErr>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(left) => {
                match other {
                    Self::Integer(right) => Ok(Self::Integer(left / right)),
                    Self::Float(right) => Ok(Self::Float(left as f64 / right)),
                    Self::String(_) => Err(TreeWalkerErr::DivideIntegerByString),
                }
            }
            Self::Float(left) => {
                match other {
                    Self::Integer(right) => Ok(Self::Float(left / right as f64)),
                    Self::Float(right) => Ok(Self::Float(left / right)),
                    Self::String(_) => Err(TreeWalkerErr::DivideFloatByString),
                }
            }
            Self::String(_) => {
                match other {
                    Self::Integer(_) => Err(TreeWalkerErr::DivideStringByInteger),
                    Self::Float(_) => Err(TreeWalkerErr::DivideStringByFloat),
                    Self::String(_) => Err(TreeWalkerErr::DivideStringByString),
                }
            }
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, TreeWalkerErr>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(left) => {
                match other {
                    Self::Integer(right) => Ok(Self::Integer(left * right)),
                    Self::Float(right) => Ok(Self::Float(left as f64 * right)),
                    Self::String(_) => Err(TreeWalkerErr::MultiplyIntegerByString),
                }
            }
            Self::Float(left) => {
                match other {
                    Self::Integer(right) => Ok(Self::Float(left * right as f64)),
                    Self::Float(right) => Ok(Self::Float(left * right)),
                    Self::String(_) => Err(TreeWalkerErr::MultiplyFloatByString),
                }
            }
            Self::String(_) => {
                match other {
                    Self::Integer(_) => Err(TreeWalkerErr::MultiplyStringByInteger),
                    Self::Float(_) => Err(TreeWalkerErr::MultiplyStringByFloat),
                    Self::String(_) => Err(TreeWalkerErr::MultiplyStringByString),
                }
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           Self::Integer(val) => write!(f, "{}", val),
           Self::Float(val) => write!(f, "{}", val),
           Self::String(val) => write!(f, "{}", val),
       } 
    }
}
