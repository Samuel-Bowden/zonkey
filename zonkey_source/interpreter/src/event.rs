use crate::element::{Button, Input, Page};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum InterpreterEvent {
    Update,
    AddPage(Arc<Mutex<Page>>),
}

#[derive(Debug, Clone)]
pub enum BrowserEvent {
    ButtonPress(Arc<Mutex<Button>>),
    InputConfirmed(Arc<Mutex<Input>>),
}
