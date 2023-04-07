use super::{message::Message, PageViewer};
use ui::event::*;

impl PageViewer {
    pub fn update(&self, message: Message) -> Option<TabEvent> {
        match message {
            Message::ButtonPressed(id) => {
                if let Some(sender) = &self.page_event_sender {
                    sender.send(PageEvent::ButtonPress(id)).unwrap();
                }
                None
            }
            Message::HyperlinkPressed(location) => Some(TabEvent::HyperlinkPressed(location)),
            Message::InputChanged(text, input) => {
                input.lock().unwrap().text = text;
                None
            }
            Message::InputSubmit(input) => {
                if let Some(sender) = &self.page_event_sender {
                    sender.send(PageEvent::InputConfirmed(input)).unwrap();
                }
                None
            }
        }
    }
}
