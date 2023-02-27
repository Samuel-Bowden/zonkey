#[derive(Debug, Clone)]
pub enum ValueType {
    Integer,
    Float,
    String,
    Boolean,
    Class(String),
}
