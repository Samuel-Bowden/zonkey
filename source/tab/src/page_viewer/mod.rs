use std::sync::{mpsc::Sender, Arc, Mutex};
use ui::{element::Page, event::PageEvent};

pub mod message;
pub mod update;
pub mod view;

pub enum PageErr {
    LoadAddressError(String),
    ScriptError(String),
}

pub struct PageViewer {
    page: Option<Arc<Mutex<Page>>>,
    page_event_sender: Option<Sender<PageEvent>>,
    page_error: Option<PageErr>,
}

impl PageViewer {
    pub fn new(
        page: Option<Arc<Mutex<Page>>>,
        page_event_sender: Option<Sender<PageEvent>>,
    ) -> Self {
        Self {
            page,
            page_event_sender,
            page_error: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            page: None,
            page_event_sender: None,
            page_error: None,
        }
    }

    pub fn finish(&mut self) {
        self.page_event_sender = None;
    }

    pub fn set_page(&mut self, page: Arc<Mutex<Page>>) {
        self.page = Some(page);
    }

    pub fn script_error(&mut self, error: String) {
        self.page_error = Some(PageErr::ScriptError(error));
    }

    pub fn load_address_error(&mut self, error: String) {
        self.page_error = Some(PageErr::LoadAddressError(error));
    }

    pub fn title(&self) -> String {
        if let Some(page) = &self.page {
            page.lock().unwrap().title.to_string()
        } else {
            if let Some(error) = &self.page_error {
                match error {
                    PageErr::LoadAddressError(_) => "Load Address Error".into(),
                    PageErr::ScriptError(_) => "Script Error".into(),
                }
            } else {
                "Loading page".into()
            }
        }
    }
}
