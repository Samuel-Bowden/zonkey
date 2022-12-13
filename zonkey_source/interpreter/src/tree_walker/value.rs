use super::err::TreeWalkerErr;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    Integer,
    Float,
    String,
    Boolean,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "an integer"),
            Self::Float => write!(f, "a float"),
            Self::String => write!(f, "a string"),
            Self::Boolean => write!(f, "a boolean"),
        }
    }
}

impl Add for Value {
    type Output = Result<Self, TreeWalkerErr>;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(left) => match other {
                Self::Integer(right) => Ok(Self::Integer(left + right)),
                Self::Float(_) => Err(TreeWalkerErr::AddErr(ValueType::Integer, ValueType::Float)),
                Self::String(_) => {
                    Err(TreeWalkerErr::AddErr(ValueType::Integer, ValueType::String))
                }
                Self::Boolean(_) => Err(TreeWalkerErr::AddErr(
                    ValueType::Integer,
                    ValueType::Boolean,
                )),
            },
            Self::Float(left) => match other {
                Self::Integer(_) => {
                    Err(TreeWalkerErr::AddErr(ValueType::Float, ValueType::Integer))
                }
                Self::Float(right) => Ok(Self::Float(left + right)),
                Self::String(_) => Err(TreeWalkerErr::AddErr(ValueType::Float, ValueType::String)),
                Self::Boolean(_) => {
                    Err(TreeWalkerErr::AddErr(ValueType::Float, ValueType::Boolean))
                }
            },
            Self::String(left) => match other {
                Self::Integer(_) => {
                    Err(TreeWalkerErr::AddErr(ValueType::String, ValueType::Integer))
                }
                Self::Float(_) => Err(TreeWalkerErr::AddErr(ValueType::String, ValueType::Float)),
                Self::String(right) => Ok(Self::String(left + &right)),
                Self::Boolean(_) => {
                    Err(TreeWalkerErr::AddErr(ValueType::String, ValueType::Boolean))
                }
            },
            Self::Boolean(_) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::AddErr(
                    ValueType::Boolean,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::AddErr(ValueType::Boolean, ValueType::Float)),
                Self::String(_) => {
                    Err(TreeWalkerErr::AddErr(ValueType::Boolean, ValueType::String))
                }
                Self::Boolean(_) => Err(TreeWalkerErr::AddErr(
                    ValueType::Boolean,
                    ValueType::Boolean,
                )),
            },
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, TreeWalkerErr>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(left) => match other {
                Self::Integer(right) => Ok(Self::Integer(left - right)),
                Self::Float(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Integer,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Integer,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Integer,
                    ValueType::Boolean,
                )),
            },
            Self::Float(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Float,
                    ValueType::Integer,
                )),
                Self::Float(right) => Ok(Self::Float(left - right)),
                Self::String(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Float,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Float,
                    ValueType::Boolean,
                )),
            },
            Self::String(_) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::String,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::String,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::String,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::String,
                    ValueType::Boolean,
                )),
            },
            Self::Boolean(_) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Boolean,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Boolean,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Boolean,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::SubtractErr(
                    ValueType::Boolean,
                    ValueType::Boolean,
                )),
            },
        }
    }
}

impl Div for Value {
    type Output = Result<Self, TreeWalkerErr>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(left) => match other {
                Self::Integer(right) => Ok(Self::Integer(left / right)),
                Self::Float(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Integer,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Integer,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Integer,
                    ValueType::Boolean,
                )),
            },
            Self::Float(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Float,
                    ValueType::Integer,
                )),
                Self::Float(right) => Ok(Self::Float(left / right)),
                Self::String(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Float,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Float,
                    ValueType::Boolean,
                )),
            },
            Self::String(_) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::String,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::String,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::String,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::String,
                    ValueType::Boolean,
                )),
            },
            Self::Boolean(_) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Boolean,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Boolean,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Boolean,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::DivideErr(
                    ValueType::Boolean,
                    ValueType::Boolean,
                )),
            },
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, TreeWalkerErr>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(left) => match other {
                Self::Integer(right) => Ok(Self::Integer(left * right)),
                Self::Float(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Integer,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Integer,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Integer,
                    ValueType::Boolean,
                )),
            },
            Self::Float(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Float,
                    ValueType::Integer,
                )),
                Self::Float(right) => Ok(Self::Float(left * right)),
                Self::String(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Float,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Float,
                    ValueType::Boolean,
                )),
            },
            Self::String(_) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::String,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::String,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::String,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::String,
                    ValueType::Boolean,
                )),
            },
            Self::Boolean(_) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Boolean,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Boolean,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Boolean,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::MultiplyErr(
                    ValueType::Boolean,
                    ValueType::Boolean,
                )),
            },
        }
    }
}

impl Value {
    pub fn equal(&self, other: &Self) -> Result<bool, TreeWalkerErr> {
        match self {
            Self::Integer(left) => match other {
                Self::Integer(right) => Ok(left == right),
                Self::Float(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Integer,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Integer,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Integer,
                    ValueType::Boolean,
                )),
            },
            Self::Float(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Float,
                    ValueType::Integer,
                )),
                Self::Float(right) => Ok(left == right),
                Self::String(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Float,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Float,
                    ValueType::Boolean,
                )),
            },
            Self::String(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::String,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::String,
                    ValueType::Float,
                )),
                Self::String(right) => Ok(left == right),
                Self::Boolean(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::String,
                    ValueType::Boolean,
                )),
            },
            Self::Boolean(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Boolean,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Boolean,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Boolean,
                    ValueType::String,
                )),
                Self::Boolean(right) => Ok(left == right),
            },
        }
    }
    pub fn less(&self, other: &Self) -> Result<bool, TreeWalkerErr> {
        match self {
            Self::Integer(left) => match other {
                Self::Integer(right) => Ok(left < right),
                Self::Float(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Integer,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Integer,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Integer,
                    ValueType::Boolean,
                )),
            },
            Self::Float(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Float,
                    ValueType::Integer,
                )),
                Self::Float(right) => Ok(left < right),
                Self::String(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Float,
                    ValueType::String,
                )),
                Self::Boolean(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Float,
                    ValueType::Boolean,
                )),
            },
            Self::String(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::String,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::String,
                    ValueType::Float,
                )),
                Self::String(right) => Ok(left < right),
                Self::Boolean(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::String,
                    ValueType::Boolean,
                )),
            },
            Self::Boolean(left) => match other {
                Self::Integer(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Boolean,
                    ValueType::Integer,
                )),
                Self::Float(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Boolean,
                    ValueType::Float,
                )),
                Self::String(_) => Err(TreeWalkerErr::EqualityErr(
                    ValueType::Boolean,
                    ValueType::String,
                )),
                Self::Boolean(right) => Ok(left < right),
            },
        }
    }
    pub fn less_equal(&self, other: &Self) -> Result<bool, TreeWalkerErr> {
        Ok(self.less(other)? || self.equal(other)?)
    }

    pub fn more(&self, other: &Self) -> Result<bool, TreeWalkerErr> {
        Ok(!(self.less_equal(other)?))
    }

    pub fn more_equal(&self, other: &Self) -> Result<bool, TreeWalkerErr> {
        Ok(!(self.less(other)?))
    }

    pub fn get_value_type(&self) -> ValueType {
        match self {
            Self::Integer(_) => ValueType::Integer,
            Self::Float(_) => ValueType::Float,
            Self::Boolean(_) => ValueType::Boolean,
            Self::String(_) => ValueType::String,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
            Self::Boolean(val) => write!(f, "{}", val),
        }
    }
}
