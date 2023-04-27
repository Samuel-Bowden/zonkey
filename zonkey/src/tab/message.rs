use interpreter::element::*;
use interpreter::{element::Page, event::PageEvent, Address};
use std::sync::{mpsc::Sender, Arc, Mutex};

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    ReadyForNextScript(Sender<Address>),
    StartedScript(Sender<PageEvent>),
    SetPage(Arc<Mutex<Page>>),
    ScriptError(String),
    LoadAddressErr(String),
    Finished,
    OpenLink(String, Vec<String>),
    ButtonPressed(Arc<Mutex<Button>>),
    HyperlinkPressed(String, Vec<String>),
    InputChanged(String, Arc<Mutex<Input>>),
    InputSubmit(Arc<Mutex<Input>>),
    None,
}
