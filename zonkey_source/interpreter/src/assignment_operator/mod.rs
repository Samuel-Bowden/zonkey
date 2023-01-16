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
