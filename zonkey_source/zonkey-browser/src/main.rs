mod component;
mod message;
mod zonkey_app;

use directories::ProjectDirs;
use iced::{
    executor, subscription,
    theme::Palette,
    widget::{Column, Container},
    Application, Color, Command, Element, Length, Settings, Theme,
};
use interpreter::event::{BrowserEvent, InterpreterEvent};
use message::Message;
use std::env::args;
use std::sync::mpsc::{Receiver, Sender};
use std::{fs::read_to_string, path::PathBuf, sync::mpsc, thread};
use unicode_segmentation::UnicodeSegmentation;
use zonkey_app::element::ElementType;
use zonkey_app::ZonkeyApp;

pub struct ZonkeyBrowser {
    app: Option<ZonkeyApp>,
    directories: ProjectDirs,
    address: String,
    sender: Option<Sender<String>>,
    interpreter_sender: Option<Sender<BrowserEvent>>,
    next_app: Option<(PathBuf, String)>,
}

impl Application for ZonkeyBrowser {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (ZonkeyBrowser, Command<Self::Message>) {
        let directories = ProjectDirs::from("rocks.sambowden", "", "zonkey-browser")
            .expect("Failed to find home directory of system");

        (
            ZonkeyBrowser {
                app: None,
                directories,
                address: String::from(""),
                sender: None,
                interpreter_sender: None,
                next_app: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        if let Some(app) = &self.app {
            format!("Zonkey Browser - {}", app.name)
        } else {
            "Zonkey Browser".to_string()
        }
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
                self.address = val;
                Command::none()
            }
            Message::AddressConfirmed => {
                self.open_address(self.address.clone());
                Command::none()
            }
            Message::HomePressed => {
                self.home_app();
                self.address = String::from("zonkey:home");
                Command::none()
            }
            Message::SettingsPressed => {
                self.settings_app();
                self.address = String::from("zonkey:settings");
                Command::none()
            }
            Message::Event(event) => {
                if let Some(app) = &mut self.app {
                    app.update(event);
                }
                Command::none()
            }
            Message::Ready(sender) => {
                self.sender = Some(sender);
                let next_app = std::mem::replace(&mut self.next_app, None);
                if let Some((path, name)) = next_app {
                    self.app(path, name);
                }
                Command::none()
            }
            Message::BootComplete(sender) => {
                self.sender = Some(sender);
                if let Some(file) = args().skip(1).next() {
                    self.open_address(file);
                } else {
                    self.home_app();
                    self.address = String::from("zonkey:home");
                }
                Command::none()
            }
            Message::PageButtonPressed(id) => {
                if let Some(sender) = &self.interpreter_sender {
                    if let Ok(()) = sender.send(BrowserEvent::ButtonPress(id)) {
                    } else {
                        println!("Interpreter ended");
                    }
                }
                Command::none()
            }
            Message::Hyperlink(location) => {
                self.open_address(location);
                Command::none()
            }
            Message::SetSender(sender) => {
                self.interpreter_sender = Some(sender);
                Command::none()
            }
            Message::InputChanged(value, id) => {
                if let Some(app) = &mut self.app {
                    if let ElementType::Page(elements) = &mut app.root {
                        if let ElementType::Input(_, _, text) = &mut elements[id as usize] {
                            *text = value;
                        }
                    }
                }
                Command::none()
            }
            Message::InputSubmit(id) => {
                if let Some(app) = &mut self.app {
                    if let ElementType::Page(elements) = &mut app.root {
                        if let ElementType::Input(_, _, text) = &mut elements[id as usize] {
                            if let Some(sender) = &self.interpreter_sender {
                                if let Err(_) =
                                    sender.send(BrowserEvent::InputConfirmed(text.to_string(), id))
                                {
                                    println!("Interpreter has ended");
                                }
                            }
                        }
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let top_bar = component::top_bar::build(&self);

        let app_viewer = if let Some(app) = &self.app {
            component::app_viewer::build(&app.root)
        } else {
            Column::new().into()
        };

        let content = Column::new()
            .push(top_bar)
            .push(app_viewer)
            .width(Length::Fill);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::unfold(0, SubscriptionState::Boot, |state| async move {
            match state {
                SubscriptionState::Boot => {
                    let (sender, receiver) = mpsc::channel();

                    (
                        Some(Message::BootComplete(sender)),
                        SubscriptionState::Ready(receiver),
                    )
                }
                SubscriptionState::Starting => {
                    let (sender, receiver) = mpsc::channel();

                    (
                        Some(Message::Ready(sender)),
                        SubscriptionState::Ready(receiver),
                    )
                }
                SubscriptionState::Ready(receiver) => {
                    let source = if let Ok(s) = receiver.recv() {
                        s
                    } else {
                        return (None, SubscriptionState::Finished);
                    };

                    let (interpreter_sender, browser_receiver) = mpsc::channel();
                    let (browser_sender, interpreter_receiver) = mpsc::channel();

                    thread::spawn(move || {
                        let graphemes = UnicodeSegmentation::graphemes(source.as_str(), true)
                            .collect::<Vec<&str>>();

                        match interpreter::run(&graphemes, interpreter_sender, interpreter_receiver)
                        {
                            Ok(_) => (),
                            Err(e) => {
                                interpreter::err::handler::run(e, &graphemes);
                            }
                        }
                    });

                    (
                        Some(Message::SetSender(browser_sender)),
                        SubscriptionState::Running(browser_receiver),
                    )
                }
                SubscriptionState::Running(receiver) => match receiver.recv() {
                    Ok(event) => (
                        Some(Message::Event(event)),
                        SubscriptionState::Running(receiver),
                    ),
                    Err(_) => (None, SubscriptionState::Starting),
                },
                SubscriptionState::Finished => (None, SubscriptionState::Finished),
            }
        })
    }
}

enum SubscriptionState {
    Boot,
    Starting,
    Ready(Receiver<String>),
    Running(Receiver<InterpreterEvent>),
    Finished,
}

impl ZonkeyBrowser {
    fn home_app(&mut self) {
        self.app(
            self.directories.data_dir().join("home.zonk"),
            "Home".to_string(),
        );
    }

    fn settings_app(&mut self) {
        self.app(
            self.directories.data_dir().join("settings.zonk"),
            "Settings".to_string(),
        );
    }

    fn invalid_app(&mut self, address: String) {
        self.app(self.directories.data_dir().join("invalid.zonk"), address);
    }

    fn app(&mut self, path: PathBuf, name: String) {
        if let Some(sender) = &self.sender {
            let source = if let Ok(s) = read_to_string(path) {
                s
            } else {
                if let Ok(s) = read_to_string(self.directories.data_dir().join("invalid.zonk")) {
                    s
                } else {
                    eprintln!("Failed to load invalid page.");
                    return;
                }
            };

            sender.send(source).expect(
                "Could not start new app as communication with interpreter died unexpectedly.",
            );

            self.sender = None;

            self.app = Some(ZonkeyApp::new_from_file(name))
        } else {
            self.interpreter_sender = None;
            self.next_app = Some((path, name));
        }
    }

    fn open_address(&mut self, address: String) {
        self.address = address.clone();

        let mut it = address.split(":");

        let mut invalid = false;

        match (it.next(), it.next(), it.next()) {
            (Some(first), Some(second), None) => match first {
                "zonkey" => match second {
                    "home" => self.home_app(),
                    "settings" => self.settings_app(),
                    _ => invalid = true,
                },
                "file" => {
                    self.app(PathBuf::from(second), "Custom app".to_string());
                }
                _ => invalid = true,
            },
            (None, None, None) => (),
            _ => invalid = true,
        }

        if invalid {
            self.invalid_app(self.address.clone());
        }
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
