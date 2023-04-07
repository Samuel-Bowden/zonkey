use crate::page_viewer;
use std::sync::{mpsc::Sender, Arc, Mutex};
use ui::{element::Page, event::PageEvent};

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    PageViewer(page_viewer::message::Message),
    ReadyForNextScript(Sender<String>),
    StartedScript(Sender<PageEvent>),
    NewPage(Arc<Mutex<Page>>),
    ScriptError,
    Finished,
}
