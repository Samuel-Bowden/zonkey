use std::sync::{mpsc::Sender, Arc, Mutex};
use ui::{element::Page, event::PageEvent};

pub mod message;
pub mod update;
pub mod view;

pub struct PageViewer {
    page: Option<Arc<Mutex<Page>>>,
    page_event_sender: Option<Sender<PageEvent>>,
}

impl PageViewer {
    pub fn new(
        page: Option<Arc<Mutex<Page>>>,
        page_event_sender: Option<Sender<PageEvent>>,
    ) -> Self {
        Self {
            page,
            page_event_sender,
        }
    }

    pub fn empty() -> Self {
        Self {
            page: None,
            page_event_sender: None,
        }
    }

    pub fn finish(&mut self) {
        self.page_event_sender = None;
    }

    pub fn set_page(&mut self, page: Arc<Mutex<Page>>) {
        self.page = Some(page);
    }

    pub fn title(&self) -> String {
        if let Some(page) = &self.page {
            page.lock().unwrap().title.clone()
        } else {
            "Loading".into()
        }
    }
}
