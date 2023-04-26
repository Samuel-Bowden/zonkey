use crate::page_viewer;
use interpreter::{element::Page, event::PageEvent, Address};
use std::sync::{mpsc::Sender, Arc, Mutex};

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    PageViewer(page_viewer::message::Message),
    ReadyForNextScript(Sender<Address>),
    StartedScript(Sender<PageEvent>),
    SetPage(Arc<Mutex<Page>>),
    ScriptError(String),
    LoadAddressErr(String),
    Finished,
    OpenLink(String, Vec<String>),
    None,
}
