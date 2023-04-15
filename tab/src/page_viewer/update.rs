use super::{message::Message, PageViewer};
use interpreter::event::*;

pub enum PageViewerEvent {
    HyperlinkPressed(String),
}

impl PageViewer {
    pub fn update(&self, message: Message) -> Option<PageViewerEvent> {
        match message {
            Message::ButtonPressed(id) => {
                if let Some(sender) = &self.page_event_sender {
                    sender.send(PageEvent::ButtonPress(id)).ok();
                }
                None
            }
            Message::HyperlinkPressed(location) => {
                Some(PageViewerEvent::HyperlinkPressed(location))
            }
            Message::InputChanged(text, input) => {
                input.lock().unwrap().text = text;
                None
            }
            Message::InputSubmit(input) => {
                if let Some(sender) = &self.page_event_sender {
                    sender.send(PageEvent::InputConfirmed(input)).ok();
                }
                None
            }
        }
    }
}
