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
