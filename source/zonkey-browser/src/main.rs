use iced::{
    executor, theme::Palette, widget::Column, Application, Color, Command, Element, Length,
    Settings, Subscription, Theme,
};
use iced_native::command::Action;
use message::Message;
use resource_loader::Address;
use std::{collections::BTreeMap, env::args, process::ExitCode};
use tab::Tab;
use ui::event::WindowEvent;

mod control_bar;
mod message;
mod tab_bar;

pub struct ZonkeyBrowser {
    tabs: BTreeMap<usize, Tab>,
    current_tab: usize,
    next_tab_id: usize,
}

impl Application for ZonkeyBrowser {
    type Executor = executor::Default;
    type Flags = Address;
    type Message = Message;
    type Theme = Theme;

    fn new(first_address: Address) -> (ZonkeyBrowser, Command<Self::Message>) {
        let mut tabs = BTreeMap::new();

        tabs.insert(0, Tab::new(first_address, 0));

        (
            ZonkeyBrowser {
                tabs,
                current_tab: 0,
                next_tab_id: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Zonkey Browser - {}", self.current_tab().title())
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::custom(Palette {
            background: Color::WHITE,
            text: Color::BLACK,
            primary: Color::from_rgb8(50, 50, 50),
            success: Color::from_rgb8(255, 255, 255),
            danger: Color::from_rgb8(0, 0, 0),
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tab((index, msg)) => {
                if let Some(tab) = self.tabs.get_mut(&index) {
                    if let Some(WindowEvent::TabFinished) = tab.update(msg) {
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
                self.current_tab_mut()
                    .open_address(Address::Zonkey("home.zonk".into()));
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
            Message::SettingsPressed => {
                self.current_tab_mut()
                    .open_address(Address::Zonkey("settings.zonk".into()));
            }
            Message::NewTab => {
                self.next_tab_id += 1;
                self.tabs.insert(
                    self.next_tab_id,
                    Tab::new(Address::Zonkey("home.zonk".into()), self.next_tab_id),
                );
                self.current_tab = self.next_tab_id;
            }
            Message::TabPressed(index) => {
                self.current_tab = index;
            }
            Message::TabClosePressed(index) => {
                self.tabs.get_mut(&index).unwrap().close();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let tab_bar = tab_bar::build(&self);

        let control_bar = control_bar::build(&self);

        let current_tab = self.current_tab().view().map(Message::Tab);

        Column::new()
            .push(tab_bar)
            .push(control_bar)
            .push(current_tab)
            .width(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        //self.current_tab().subscription().map(Message::Tab)
        Subscription::batch(
            self.tabs
                .iter()
                .map(|(_, tab)| tab.subscription().map(Message::Tab)),
        )
    }
}

impl ZonkeyBrowser {
    fn current_tab(&self) -> &Tab {
        self.tabs.get(&self.current_tab).unwrap()
    }

    fn current_tab_mut(&mut self) -> &mut Tab {
        self.tabs.get_mut(&self.current_tab).unwrap()
    }
}

pub fn main() -> ExitCode {
    let address = if let Some(address_string) = args().nth(1) {
        Address::new(&address_string)
    } else {
        Address::Zonkey("home.zonk".into())
    };

    let result = ZonkeyBrowser::run(Settings {
        default_font: Some("Noto".as_bytes()),
        antialiasing: true,
        text_multithreading: true,
        flags: address,
        id: None,
        window: iced::window::Settings::default(),
        default_text_size: 20.,
        exit_on_close_request: true,
        try_opengles_first: false,
    });

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Failure running browser UI: {e}");
            ExitCode::FAILURE
        }
    }
}
