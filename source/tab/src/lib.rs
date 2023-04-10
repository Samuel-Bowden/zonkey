use iced::{subscription, Element};
pub use message::Message;
use non_empty_vec::NonEmpty;
use page_viewer::PageViewer;
use resource_loader::Address;
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};
use subscription_state::{SubscriptionState, SubscriptionStateVariant};
use ui::{element::Page, event::*};

mod message;
mod page_viewer;
mod subscription_state;

pub type MessagePointer = (usize, Message);

pub struct Tab {
    page_viewer: PageViewer,
    script_executor_sender: Option<Sender<Address>>,
    initial_state: Arc<Mutex<SubscriptionState>>,
    history: NonEmpty<Address>,
    waiting_to_load_next_script: bool,
    pub address_field: String,
    position: usize,
    closing: bool,
}

impl Tab {
    pub fn new(address: Address, position: usize) -> Self {
        let address_field = address.to_string();
        Self {
            page_viewer: PageViewer::empty(),
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
            page_viewer: PageViewer::new(Some(page), Some(page_event_sender)),
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
        self.page_viewer
            .view()
            .map(|msg| (self.position, Message::PageViewer(msg)))
    }

    pub fn update(&mut self, message: Message) -> Option<WindowEvent> {
        match message {
            Message::Update => (),
            Message::PageViewer(msg) => match self.page_viewer.update(msg) {
                Some(TabEvent::HyperlinkPressed(location)) => {
                    self.open_address_from_string(location)
                }
                None => (),
            },
            Message::StartedScript(page_event_sender) => {
                self.page_viewer = PageViewer::new(None, Some(page_event_sender));
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
                self.page_viewer.set_page(page);
            }
            Message::ScriptError(error) => {
                self.page_viewer.script_error(error);
            }
            Message::LoadAddressErr(error) => {
                self.page_viewer.load_address_error(error);
            }
            Message::Finished => return Some(WindowEvent::TabFinished),
        }

        None
    }

    pub fn title(&self) -> String {
        self.page_viewer.title()
    }

    pub fn subscription(&self) -> iced::Subscription<MessagePointer> {
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
                        Some((index, Message::ReadyForNextScript(sender))),
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
                        return (None, (index, SubscriptionStateVariant::Finished));
                    };

                    let (interpreter_sender, tab_receiver) = mpsc::channel();
                    let (tab_sender, interpreter_receiver) = mpsc::channel();

                    thread::spawn(move || {
                        interpreter::run_with_std_stream_error_handling(
                            source,
                            interpreter_sender,
                            interpreter_receiver,
                        );
                    });

                    (
                        Some((index, Message::StartedScript(tab_sender))),
                        (index, SubscriptionStateVariant::RunningScript(tab_receiver)),
                    )
                }

                SubscriptionStateVariant::RunningScript(receiver) => match receiver.recv() {
                    Ok(event) => (
                        match event {
                            InterpreterEvent::Update => Some((index, Message::Update)),
                            InterpreterEvent::SetPage(page) => {
                                Some((index, Message::SetPage(page)))
                            }
                            InterpreterEvent::ScriptError(error) => {
                                Some((index, Message::ScriptError(error)))
                            }
                            InterpreterEvent::LoadAddressError(error) => {
                                Some((index, Message::LoadAddressErr(error)))
                            }
                            InterpreterEvent::CloseTab => {
                                Some((index, Message::Finished))
                            }
                        },
                        (index, SubscriptionStateVariant::RunningScript(receiver)),
                    ),
                    Err(_) => (
                        None,
                        (index, SubscriptionStateVariant::PreparingForNextScript),
                    ),
                },

                SubscriptionStateVariant::Finished => (
                    Some((index, Message::Finished)),
                    (index, SubscriptionStateVariant::Finished),
                ),
            }
        })
    }

    pub fn load_script(&mut self) {
        self.page_viewer.finish();

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

    fn open_address_from_string(&mut self, string: String) {
        let address = Address::new(&string);
        self.open_address(address);
    }

    pub fn open_address_in_bar(&mut self) {
        self.open_address_from_string(self.address_field.clone())
    }

    pub fn reload(&mut self) {
        self.load_script()
    }

    pub fn back(&mut self) {
        self.history.pop();
        self.load_script()
    }

    pub fn close(&mut self) {
        self.page_viewer.finish();
        self.script_executor_sender = None;
        self.closing = true;
    }

    pub fn open_address(&mut self, address: Address) {
        self.history.push(address);
        self.load_script()
    }

    pub fn history(&self) -> &NonEmpty<Address> {
        &self.history
    }
}
