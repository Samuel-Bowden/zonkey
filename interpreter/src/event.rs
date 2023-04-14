use super::element::{Button, Input, Page};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum InterpreterEvent {
    SetPage(Arc<Mutex<Page>>),
    Update,
    ScriptError(String),
    LoadAddressError(String),
    CloseTab,
    OpenLink(String),
}

#[derive(Debug, Clone)]
pub enum PageEvent {
    ButtonPress(Arc<Mutex<Button>>),
    InputConfirmed(Arc<Mutex<Input>>),
}
