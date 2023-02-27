use crate::class::Class;

#[derive(Debug)]
pub enum TreeWalkerStatus {
    Ok,
    Continue,
    ReturnInt(i64),
    ReturnFloat(f64),
    ReturnString(String),
    ReturnBoolean(bool),
    ReturnClass(Class),
    ReturnNone,
    Break,
}
