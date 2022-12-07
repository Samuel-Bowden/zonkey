use std::fmt::Display;

#[derive(Debug)]
pub enum TreeWalkerErr {
    UnsupportedOperator,
    AddStringToInteger,
    AddStringToFloat,
    AddFloatToString,
    AddIntegerToString,
    SubtractStringFromInteger,
    SubtractStringFromFloat,
    SubtractIntegerFromString,
    SubtractFloatFromString,
    SubtractStringFromString,
    DivideIntegerByString,
    DivideFloatByString,
    DivideStringByInteger,
    DivideStringByFloat,
    DivideStringByString,
    MultiplyIntegerByString,
    MultiplyFloatByString,
    MultiplyStringByInteger,
    MultiplyStringByFloat,
    MultiplyStringByString,
}

impl Display for TreeWalkerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedOperator => write!(f, "An unsupported token was used as an operator"),
            Self::AddStringToInteger => write!(f, "Cannot add a string to an integer"),
            Self::AddStringToFloat => write!(f, "Cannot add a string to a float"),
            Self::AddFloatToString => write!(f, "Cannot add a float to a string"),
            Self::AddIntegerToString => write!(f, "Cannot add an integer to a string"),
            Self::SubtractStringFromInteger => write!(f, "Cannot subtract a string from an integer"),
            Self::SubtractStringFromFloat => write!(f, "Cannot subtract a string from a float"),
            Self::SubtractIntegerFromString => write!(f, "Cannot subtract an integer from a string"),
            Self::SubtractFloatFromString => write!(f, "Cannot subtract a float from a string"),
            Self::SubtractStringFromString => write!(f, "Cannot subtract a string from a string"),
            Self::DivideIntegerByString => write!(f, "Cannot divide an integer by a string"),
            Self::DivideFloatByString => write!(f, "Cannot divide a float by a string"),
            Self::DivideStringByInteger => write!(f, "Cannot divide a string by an integer"),
            Self::DivideStringByFloat => write!(f, "Cannot divide a string by a float"),
            Self::DivideStringByString => write!(f, "Cannot divide a string by a string"),
            Self::MultiplyIntegerByString => write!(f, "Cannot multiply an integer by a string"),
            Self::MultiplyFloatByString => write!(f, "Cannot multiply a float by a string"),
            Self::MultiplyStringByInteger => write!(f, "Cannot multiply a string by an integer"),
            Self::MultiplyStringByFloat => write!(f, "Cannot multiply a string by a float"),
            Self::MultiplyStringByString => write!(f, "Cannot multiply a string by a string"),
        }
    }
}
