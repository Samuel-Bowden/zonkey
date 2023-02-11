use iced::widget::{container, text, text_input, Column};
use iced::{executor, Length};
use iced::{Alignment, Application, Command, Element, Theme};
use interpreter::err;
use std::fs::read_to_string;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

pub struct ZonkeyBrowser {
    program_location_value: String,
    running_script: bool,
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ProgramLocationChanged(String),
    ProgramLocationConfirmed,
    ScriptComplete,
    ScriptFailed,
}

impl Application for ZonkeyBrowser {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (ZonkeyBrowser, Command<Self::Message>) {
        (
            ZonkeyBrowser {
                program_location_value: String::new(),
                running_script: false,
                error_message: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Zonkey Browser")
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ProgramLocationChanged(val) => {
                if !self.running_script {
                    self.program_location_value = val;
                }

                Command::none()
            }
            Message::ProgramLocationConfirmed => {
                let source = match read_to_string(Path::new(&self.program_location_value)) {
                    Ok(s) => s,
                    Err(e) => {
                        self.error_message = Some(format!("Couldn't open file: {e}"));
                        return Command::none();
                    }
                };

                self.error_message = None;
                self.running_script = true;

                Command::perform(run_script(source), |result| {
                    if result {
                        Message::ScriptComplete
                    } else {
                        Message::ScriptFailed
                    }
                })
            }
            Message::ScriptComplete => {
                self.running_script = false;
                Command::none()
            }
            Message::ScriptFailed => {
                self.running_script = false;
                self.error_message = Some(format!("Error: Program has crashed"));
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let program_location = text_input(
            "Enter address...",
            &self.program_location_value,
            Message::ProgramLocationChanged,
        )
        .padding(10)
        .on_submit(Message::ProgramLocationConfirmed)
        .size(20);

        let mut children = vec![text("Search").into(), program_location.into()];

        if self.running_script {
            children.push(text("Running program").into());
        }

        if let Some(msg) = &self.error_message {
            children.push(text(msg).into());
        }

        let content = Column::with_children(children)
            .align_items(Alignment::Center)
            .max_width(500)
            .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

async fn run_script(source: String) -> bool {
    let graphemes = UnicodeSegmentation::graphemes(source.as_str(), true).collect::<Vec<&str>>();

    match interpreter::run(&graphemes) {
        Ok(_) => true,
        Err(e) => {
            err::handler::run(e, &graphemes);
            false
        }
    }
}
