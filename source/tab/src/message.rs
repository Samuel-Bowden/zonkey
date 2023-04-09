use crate::page_viewer;
use resource_loader::Address;
use std::sync::{mpsc::Sender, Arc, Mutex};
use ui::{element::Page, event::PageEvent};

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    PageViewer(page_viewer::message::Message),
    ReadyForNextScript(Sender<Address>),
    StartedScript(Sender<PageEvent>),
    NewPage(Arc<Mutex<Page>>),
    ScriptError(String),
    LoadAddressErr(String),
    Finished,
}
