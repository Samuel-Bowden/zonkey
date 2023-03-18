use super::environment::Environment;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum TreeWalkerStatus {
    Ok,
    Continue,
    ReturnInt(i64),
    ReturnFloat(f64),
    ReturnString(String),
    ReturnBoolean(bool),
    ReturnObject(Rc<RefCell<Environment>>),
    ReturnNone,
    Break,
}
