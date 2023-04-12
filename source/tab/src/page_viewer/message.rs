use std::sync::{Arc, Mutex};
use interpreter::element::*;

#[derive(Clone, Debug)]
pub enum Message {
    ButtonPressed(Arc<Mutex<Button>>),
    HyperlinkPressed(String),
    InputChanged(String, Arc<Mutex<Input>>),
    InputSubmit(Arc<Mutex<Input>>),
}
