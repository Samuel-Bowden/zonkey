use iced::{executor, Application, Command, Renderer, Theme};
use resource_loader::Address;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};
use tab::Tab;
use ui::{
    element::Page,
    event::{InterpreterEvent, PageEvent},
};

pub struct Window {
    tab: Tab,
}

impl Application for Window {
    type Executor = executor::Default;
    type Flags = (
        Arc<Mutex<Page>>,
        Sender<PageEvent>,
        Receiver<InterpreterEvent>,
        Address,
    );
    type Message = tab::MessagePointer;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (page, page_event_sender, interpreter_event_receiver, address) = flags;

        (
            Self {
                tab: Tab::new_from_running_script(
                    page,
                    page_event_sender,
                    interpreter_event_receiver,
                    address,
                ),
            },
            Command::none(),
        )
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        self.tab.view()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let (_, msg) = message;
        self.tab.update(msg);
        Command::none()
    }

    fn title(&self) -> String {
        self.tab.title()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        self.tab.subscription()
    }
}
