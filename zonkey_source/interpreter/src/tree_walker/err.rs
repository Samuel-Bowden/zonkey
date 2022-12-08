use std::fmt::Display;

use super::value::ValueType;

#[derive(Debug)]
pub enum TreeWalkerErr {
    UnsupportedOperator,
    AddErr(ValueType, ValueType),
    MultiplyErr(ValueType, ValueType),
    SubtractErr(ValueType, ValueType),
    DivideErr(ValueType, ValueType),
    EqualityErr(ValueType, ValueType),
}

impl Display for TreeWalkerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedOperator => write!(f, "An unsupported token was used as an operator"),
            Self::AddErr(left, right) => write!(f, "Cannot add {right} to {left}"),
            Self::SubtractErr(left, right) => write!(f, "Cannot subtract {right} from {left}"),
            Self::DivideErr(left, right) => write!(f, "Cannot divide {left} by {right}"),
            Self::MultiplyErr(left, right) => write!(f, "Cannot multiply {left} by {right}"),
            Self::EqualityErr(left, right) => write!(f, "Cannot compare {left} to {right}"),
        }
    }
}
