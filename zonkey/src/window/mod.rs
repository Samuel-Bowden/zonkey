use crate::tab::iced;
use crate::tab::iced_native;
use crate::tab::iced_native::color;
use crate::tab::Address;
use crate::tab::{Tab, TabEvent};
use interpreter::address::AddressType;
use interpreter::element::Page;
use interpreter::event::{InterpreterEvent, PageEvent};
use interpreter::iced::{
    executor, theme::Palette, widget::Column, Application, Color, Command, Element, Length,
    Subscription, Theme,
};
use interpreter::iced_native::command::Action;
use message::Message;
use std::collections::BTreeMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

mod control_bar;
mod message;
mod tab_bar;

pub struct Window {
    tabs: BTreeMap<usize, Tab>,
    browser_gui: bool,
    current_tab: usize,
    next_tab_id: usize,
    zoom_level: f64,
}

impl Application for Window {
    type Executor = executor::Default;
    type Flags = (
        Address,
        Option<(
            Arc<Mutex<Page>>,
            Sender<PageEvent>,
            Receiver<InterpreterEvent>,
        )>,
    );
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Window, Command<Self::Message>) {
        let mut tabs = BTreeMap::new();

        let (address, running_page) = flags;

        let mut browser_gui = false;

        if let Some(running_page) = running_page {
            let (page, page_event_sender, interpreter_event_receiver) = running_page;

            tabs.insert(
                0,
                Tab::new_from_running_script(
                    page,
                    page_event_sender,
                    interpreter_event_receiver,
                    address,
                ),
            );
        } else {
            tabs.insert(0, Tab::new(address, 0));
            browser_gui = true;
        }

        (
            Window {
                tabs,
                browser_gui,
                current_tab: 0,
                next_tab_id: 0,
                zoom_level: 1.0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        if self.browser_gui {
            format!("Zonkey Browser - {}", self.current_tab().title())
        } else {
            self.current_tab().title()
        }
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::custom(Palette {
            background: Color::WHITE,
            primary: color!(0xe1e2e2),
            text: Color::BLACK,
            success: Color::from_rgb8(255, 255, 255),
            danger: Color::from_rgb8(0, 0, 0),
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tab((index, msg)) => {
                if let Some(tab) = self.tabs.get_mut(&index) {
                    if let Some(TabEvent::Finished) = tab.update(msg) {
                        if self.tabs.len() > 1 {
                            if index == self.current_tab {
                                // Move focus to right of tab to be closed
                                let pos = self.tabs.iter().position(|t| *t.0 == index).unwrap();
                                self.tabs.remove(&index);
                                // Make sure that this isn't the last tab
                                if let Some(new_id) = self.tabs.iter().nth(pos) {
                                    self.current_tab = *new_id.0;
                                } else {
                                    self.current_tab = *self.tabs.last_entry().unwrap().key();
                                }
                            } else {
                                self.tabs.remove(&index);
                            }
                        } else {
                            // This is the last tab - close the application
                            return Command::single(Action::Window(
                                iced_native::window::Action::Close,
                            ));
                        }
                    }
                }
            }
            Message::HomePressed => {
                self.current_tab_mut().open_address(Address {
                    address_type: AddressType::Zonkey,
                    location: "home.zonk".into(),
                    arguments: vec![],
                });
            }
            Message::SettingsPressed => {
                use std::env::consts;
                self.current_tab_mut().open_address(Address {
                    address_type: AddressType::Zonkey,
                    location: "settings.zonk".into(),
                    arguments: vec![
                        consts::OS.to_string(),
                        consts::ARCH.to_string(),
                        env!("CARGO_PKG_VERSION").to_string(),
                        env!("CARGO_PKG_AUTHORS").to_string(),
                    ],
                });
            }
            Message::ReloadPressed => {
                self.current_tab_mut().reload();
            }
            Message::BackPressed => {
                self.current_tab_mut().back();
            }
            Message::AddressChanged(new_value) => {
                self.current_tab_mut().address_field = new_value;
            }
            Message::AddressConfirmed => {
                self.current_tab_mut().open_address_in_bar();
            }
            Message::NewTab => {
                if self.tabs.len() < 8 {
                    self.next_tab_id += 1;
                    self.tabs.insert(
                        self.next_tab_id,
                        Tab::new(
                            Address {
                                address_type: AddressType::Zonkey,
                                location: "home.zonk".into(),
                                arguments: vec![],
                            },
                            self.next_tab_id,
                        ),
                    );
                    self.current_tab = self.next_tab_id;
                }
            }
            Message::TabPressed(index) => {
                self.current_tab = index;
            }
            Message::TabClosePressed(index) => {
                self.tabs.get_mut(&index).unwrap().close();
            }
            Message::ZoomIn => {
                if self.zoom_level < 2.0 {
                    self.zoom_level += 0.1;
                }
            }
            Message::ZoomOut => {
                if self.zoom_level > 0.5 {
                    self.zoom_level -= 0.1;
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let current_tab = self.current_tab().view().map(Message::Tab);

        let mut column = Column::new().width(Length::Fill);

        if self.browser_gui {
            let tab_bar = self.tab_bar_build();

            let control_bar = self.control_bar_build();

            column = column.push(tab_bar).push(control_bar);
        }

        column.push(current_tab).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::batch(
            self.tabs
                .iter()
                .map(|(_, tab)| tab.subscription().map(Message::Tab)),
        )
    }

    fn scale_factor(&self) -> f64 {
        self.zoom_level
    }
}

impl Window {
    fn current_tab(&self) -> &Tab {
        self.tabs.get(&self.current_tab).unwrap()
    }

    fn current_tab_mut(&mut self) -> &mut Tab {
        self.tabs.get_mut(&self.current_tab).unwrap()
    }
}
