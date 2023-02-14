mod app;
mod component;
mod default_app;
mod message;

use std::{path::Path, fs::read_to_string};

use app::{App, element::ElementType};
use iced::{
    executor,
    theme::Palette,
    widget::{Column, Container},
    Application, Color, Command, Element, Length, Settings, Theme,
};
use message::Message;

pub struct ZonkeyBrowser {
    app: App,
}

impl Application for ZonkeyBrowser {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (ZonkeyBrowser, Command<Self::Message>) {
        (
            ZonkeyBrowser {
                app: default_app::home::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Zonkey Browser - {}", self.app.name)
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
            Message::AddressChanged(val) => {
                self.app.address = val;
                Command::none()
            }
            Message::AddressConfirmed => {
                let mut it = self.app.address.split(":");

                let mut invalid = false;
                
                match (it.next(), it.next(), it.next()) {
                    (Some(first), Some(second), None) => {
                        match first {
                            "zonkey" => {
                                match second {
                                    "settings" => self.app = default_app::settings::new(),
                                    "home" => self.app = default_app::home::new(),
                                    _ => invalid = true,
                                }
                            }
                            "file" => {
                                match read_to_string(Path::new(second)) {
                                    Ok(_) => {
                                        self.app = App {
                                            name: String::from("Test"),
                                            address: std::mem::take(&mut self.app.address),
                                            root: ElementType::None,
                                        }
                                    }
                                    Err(_) => {
                                        invalid = true;
                                    }
                                };
                            }
                            _ => invalid = true,
                        }
                    }
                    (None, None, None) => (),
                    _ => invalid = true,
                }

                if invalid {
                    self.app = default_app::invalid::new(self.app.address.clone());
                }

                Command::none()
            }
            Message::HomePressed => {
                self.app = default_app::home::new();
                Command::none()
            }
            Message::SettingsPressed => {
                self.app = default_app::settings::new();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let top_bar = component::top_bar::build(&self);

        let app_viewer = component::app_viewer::build(&self.app.root);

        let content = Column::new()
            .push(top_bar)
            .push(app_viewer)
            .width(Length::Fill);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn main() -> iced::Result {
    ZonkeyBrowser::run(Settings {
        default_font: Some("Noto".as_bytes()),
        antialiasing: true,
        text_multithreading: true,
        ..Default::default()
    })
}
