use std::fmt::Display;

use super::value::ValueType;

#[derive(Debug, PartialEq)]
pub enum TreeWalkerErr {
    UnsupportedOperator,
    AddErr(ValueType, ValueType),
    MultiplyErr(ValueType, ValueType),
    SubtractErr(ValueType, ValueType),
    DivideErr(ValueType, ValueType),
    EqualityErr(ValueType, ValueType),
    VariableAssignmentIncompatibleTypes(ValueType, ValueType),
    VariableNotDefined(String),
    IfConditionMustEvaluateToBoolean,
    BreakOutsideLoop,
    StartNotInGlobalScope,
    MultipleStartDeclarations,
    InvalidCodeInGlobalScope,
    NoStartDeclaration,
    FunctionNotDefined(String),
    CallIncorrectArity(String, usize, usize),
    CallArgumentIncompatibleTypes(String, usize),
    NestedFunctionsNotAllowed,
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
            Self::VariableAssignmentIncompatibleTypes(variable_type, evaluation_type) => write!(f, "Cannot assign the evaluated value (of {evaluation_type} type) to the variable (of {variable_type} type) as they have different types"),
            Self::VariableNotDefined(name) => write!(f, "Variable '{name}' has not been defined"),
            Self::IfConditionMustEvaluateToBoolean => write!(f, "Condition of an if statement must evaluate to a boolean"),
            Self::BreakOutsideLoop => write!(f, "Break statement outside of loop"),
            Self::StartNotInGlobalScope => write!(f, "Start must be defined once in the global scope"),
            Self::MultipleStartDeclarations => write!(f, "More than one start declaration has been defined"),
            Self::InvalidCodeInGlobalScope => write!(f, "Zonkey source files must not have anything but function and start declarations in the global scope"),
            Self::NoStartDeclaration => write!(f, "No start declaration found"),
            Self::FunctionNotDefined(name) => write!(f, "Function '{name}' has not been defined"),
            Self::CallIncorrectArity(name, expected, given) => write!(f, "Call '{name}' expected {expected} arguments but was given {given}"),
            Self::CallArgumentIncompatibleTypes(name, arg_num) => write!(f, "Call '{name}' received an incorrect type for argument {arg_num}"),
            Self::NestedFunctionsNotAllowed => write!(f, "Nested functions are not allowed in Zonkey"),
        }
    }
}
