#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Integer,
    Float,
    String,
    Boolean,
    Class(String),
    None,
}
