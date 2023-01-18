#[derive(Debug, PartialEq)]
pub enum TreeWalkerStatus {
    Ok,
    Continue,
    ReturnInt(i64),
    ReturnFloat(f64),
    ReturnString(String),
    ReturnBoolean(bool),
    ReturnNone,
    Break,
}
