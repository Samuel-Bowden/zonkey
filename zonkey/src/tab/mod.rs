pub use interpreter::iced;
use interpreter::iced::subscription;
use interpreter::iced::widget::text;
use interpreter::iced::widget::Container;
use interpreter::iced::Element;
pub use interpreter::iced_native;
use interpreter::iced_native::alignment::Horizontal;
use interpreter::iced_native::alignment::Vertical;
use interpreter::iced_native::Length;
pub use interpreter::Address;
pub use interpreter::{
    element::Page,
    event::{InterpreterEvent, PageEvent},
};
pub use message::Message;
use non_empty_vec::NonEmpty;
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};
use subscription_state::{SubscriptionState, SubscriptionStateVariant};

mod message;
mod page_builder;
mod subscription_state;

pub type MessagePointer = (usize, Message);

pub enum TabEvent {
    Finished,
}

pub enum PageErr {
    LoadAddressError(String),
    ScriptError(String),
}

pub struct Tab {
    page: Option<Arc<Mutex<Page>>>,
    page_event_sender: Option<Sender<PageEvent>>,
    page_error: Option<PageErr>,
    script_executor_sender: Option<Sender<Address>>,
    initial_state: Arc<Mutex<SubscriptionState>>,
    pub history: NonEmpty<Address>,
    waiting_to_load_next_script: bool,
    pub address_field: String,
    position: usize,
    closing: bool,
}

impl Tab {
    pub fn new(address: Address, position: usize) -> Self {
        let address_field = address.to_string();
        Self {
            page: None,
            page_event_sender: None,
            page_error: None,
            script_executor_sender: None,
            waiting_to_load_next_script: true,
            initial_state: Arc::new(Mutex::new((
                position,
                SubscriptionStateVariant::PreparingForNextScript,
            ))),
            history: NonEmpty::new(address),
            address_field,
            position,
            closing: false,
        }
    }

    pub fn new_from_running_script(
        page: Arc<Mutex<Page>>,
        page_event_sender: Sender<PageEvent>,
        interpreter_event_receiver: Receiver<InterpreterEvent>,
        current_script: Address,
    ) -> Self {
        let address_field = current_script.to_string();
        Self {
            page: Some(page),
            page_event_sender: Some(page_event_sender),
            page_error: None,
            script_executor_sender: None,
            initial_state: Arc::new(Mutex::new((
                0,
                SubscriptionStateVariant::RunningScript(interpreter_event_receiver),
            ))),
            history: NonEmpty::new(current_script),
            waiting_to_load_next_script: false,
            address_field,
            position: 0,
            closing: false,
        }
    }

    pub fn view(&self) -> Element<MessagePointer> {
        if let Some(error) = &self.page_error {
            return match error {
                PageErr::ScriptError(error) => page_builder::script_error_page(error),
                PageErr::LoadAddressError(error) => page_builder::load_address_error_page(error),
            }
            .map(|msg| (self.position, msg));
        }

        if let Some(page) = &self.page {
            page_builder::build_page(page).map(|msg| (self.position, msg))
        } else {
            Container::new(text("Loading page").size(40))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }

    pub fn update(&mut self, message: Message) -> Option<TabEvent> {
        match message {
            Message::Update => (),
            Message::ButtonPressed(id) => {
                if let Some(sender) = &self.page_event_sender {
                    sender.send(PageEvent::ButtonPress(id)).ok();
                }
            }
            Message::HyperlinkPressed(location, arguments) => {
                self.open_address_from_string(location, arguments)
            }
            Message::InputChanged(text, input) => {
                input.lock().unwrap().text = text;
            }
            Message::InputSubmit(input) => {
                if let Some(sender) = &self.page_event_sender {
                    sender.send(PageEvent::InputConfirmed(input)).ok();
                }
            }
            Message::StartedScript(page_event_sender) => {
                self.page = None;
                self.page_event_sender = Some(page_event_sender);
                self.page_error = None;
            }
            Message::ReadyForNextScript(script_executor_sender) => {
                if !self.closing {
                    self.script_executor_sender = Some(script_executor_sender);

                    if self.waiting_to_load_next_script {
                        self.load_script();
                        self.waiting_to_load_next_script = false;
                    }
                }
            }
            Message::SetPage(page) => {
                self.page = Some(page);
            }
            Message::ScriptError(error) => {
                self.page_error = Some(PageErr::ScriptError(error));
            }
            Message::LoadAddressErr(error) => {
                self.page_error = Some(PageErr::LoadAddressError(error));
            }
            Message::Finished => return Some(TabEvent::Finished),
            Message::OpenLink(link, arguments) => self.open_address_from_string(link, arguments),
            Message::None => (),
        }

        None
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

    pub fn subscription(&self) -> interpreter::iced::Subscription<MessagePointer> {
        let mut set_initial_state = self.initial_state.lock().unwrap();
        let initial_state = std::mem::replace(
            &mut *set_initial_state,
            (0, SubscriptionStateVariant::Finished),
        );

        subscription::unfold(self.position, initial_state, |state| async move {
            let (index, variant) = state;
            match variant {
                SubscriptionStateVariant::PreparingForNextScript => {
                    let (sender, receiver) = mpsc::channel();

                    (
                        (index, Message::ReadyForNextScript(sender)),
                        (
                            index,
                            SubscriptionStateVariant::ReadyForNextScript(receiver),
                        ),
                    )
                }

                SubscriptionStateVariant::ReadyForNextScript(receiver) => {
                    let source = if let Ok(s) = receiver.recv() {
                        s
                    } else {
                        return (
                            (index, Message::Finished),
                            (index, SubscriptionStateVariant::PreparingForNextScript),
                        );
                    };

                    let (interpreter_sender, tab_receiver) = mpsc::channel();
                    let (tab_sender, interpreter_receiver) = mpsc::channel();

                    thread::spawn(move || {
                        interpreter::run_with_error_messages(
                            source,
                            interpreter_sender,
                            interpreter_receiver,
                        );
                    });

                    (
                        (index, Message::StartedScript(tab_sender)),
                        (index, SubscriptionStateVariant::RunningScript(tab_receiver)),
                    )
                }

                SubscriptionStateVariant::RunningScript(receiver) => match receiver.recv() {
                    Ok(event) => (
                        match event {
                            InterpreterEvent::Update => (index, Message::Update),
                            InterpreterEvent::SetPage(page) => (index, Message::SetPage(page)),
                            InterpreterEvent::ScriptError(error) => {
                                (index, Message::ScriptError(error))
                            }
                            InterpreterEvent::LoadAddressError(error) => {
                                (index, Message::LoadAddressErr(error))
                            }
                            InterpreterEvent::CloseTab => (index, Message::Finished),
                            InterpreterEvent::OpenLink(link, arguments) => {
                                (index, Message::OpenLink(link, arguments))
                            }
                        },
                        (index, SubscriptionStateVariant::RunningScript(receiver)),
                    ),
                    Err(_) => (
                        (index, Message::None),
                        (index, SubscriptionStateVariant::PreparingForNextScript),
                    ),
                },

                SubscriptionStateVariant::Finished => (
                    (index, Message::None),
                    (index, SubscriptionStateVariant::Finished),
                ),
            }
        })
    }

    pub fn load_script(&mut self) {
        // Finish currently running script
        self.page_event_sender = None;

        if let Some(sender) = std::mem::take(&mut self.script_executor_sender) {
            let address = self.history.last();
            self.address_field = address.to_string();
            sender.send(address.clone()).expect(
                "Could not start new app as communication with interpreter died unexpectedly.",
            );
        } else {
            self.waiting_to_load_next_script = true;
        }
    }

    fn open_address_from_string(&mut self, string: String, arguments: Vec<String>) {
        let address = Address::new(&string, arguments);
        self.open_address(address);
    }

    pub fn open_address_in_bar(&mut self) {
        self.open_address_from_string(self.address_field.clone(), vec![])
    }

    pub fn reload(&mut self) {
        self.load_script()
    }

    pub fn back(&mut self) {
        self.history.pop();
        self.load_script()
    }

    pub fn close(&mut self) {
        // Finish currently running script
        self.page_event_sender = None;

        self.script_executor_sender = None;
        self.closing = true;
    }

    pub fn open_address(&mut self, address: Address) {
        self.history.push(address);
        self.load_script()
    }
}
