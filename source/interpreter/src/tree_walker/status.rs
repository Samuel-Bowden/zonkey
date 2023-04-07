use super::object::Object;

#[derive(Debug)]
pub enum TreeWalkerStatus {
    Ok,
    Continue,
    ReturnInt(i64),
    ReturnFloat(f64),
    ReturnString(String),
    ReturnBoolean(bool),
    ReturnObject(Object),
    ReturnNone,
    Break,
}
