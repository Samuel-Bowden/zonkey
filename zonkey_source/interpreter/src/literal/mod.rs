#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
}
