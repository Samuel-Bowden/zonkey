use super::element::{Button, Input, Page};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum InterpreterEvent {
    NewPage(Arc<Mutex<Page>>),
    Update,
    ScriptError,
}

pub enum WindowEvent {
    TabFinished,
}

pub enum TabEvent {
    HyperlinkPressed(String),
}

#[derive(Debug, Clone)]
pub enum PageEvent {
    ButtonPress(Arc<Mutex<Button>>),
    InputConfirmed(Arc<Mutex<Input>>),
}
