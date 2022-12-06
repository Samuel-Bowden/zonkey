use std::{ops::{Sub, Add}, fmt::Display};

pub enum Value {
    Integer(i64),
    Float(f64),
}

impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
         match self {
            Self::Integer(left) => {
                match other {
                    Self::Integer(right) => Self::Integer(left + right),
                    Self::Float(right) => Self::Float(left as f64 + right),
                }
            }
            Self::Float(left) => {
                match other {
                    Self::Integer(right) => Self::Float(left + right as f64),
                    Self::Float(right) => Self::Float(left + right),
                }
            }
        }

    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::Integer(left) => {
                match other {
                    Self::Integer(right) => Self::Integer(left - right),
                    Self::Float(right) => Self::Float(left as f64 - right),
                }
            }
            Self::Float(left) => {
                match other {
                    Self::Integer(right) => Self::Float(left - right as f64),
                    Self::Float(right) => Self::Float(left - right),
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
       } 
    }
}
