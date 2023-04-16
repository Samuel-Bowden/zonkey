use crate::tab::Address;
use clap::Parser;
use interpreter::{
    event::InterpreterEvent,
    iced::{self, Application, Settings},
};
use std::{process::ExitCode, sync::mpsc, thread};
use window::Window;

mod page_viewer;
mod tab;
mod window;

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
struct Arguments {
    ///A script address can be a file path, e.g. 'folder/test.zonk', or a zonkey formatted address, e.g. 'zonkey:home.zonk', 'https://localhost:8000/test.zonk'
    script_address: String,

    #[arg(short, long)]
    ///Launch browser - provide the script address 'zonkey:home.zonk' to load the home page
    browser: bool,
}

pub fn main() -> ExitCode {
    let arguments = Arguments::parse();
    let address = Address::new(&arguments.script_address);

    if arguments.browser {
        browser(address)
    } else {
        command_line_tool(address)
    }
}

fn browser(address: Address) -> ExitCode {
    let result = Window::run(Settings {
        default_font: Some("Noto".as_bytes()),
        antialiasing: true,
        text_multithreading: true,
        flags: (address, None),
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

fn command_line_tool(address: Address) -> ExitCode {
    let (interpreter_event_sender, interpreter_event_receiver) = mpsc::channel();
    let (page_event_sender, page_event_receiver) = mpsc::channel();

    let address_copy = address.clone();

    thread::spawn(move || {
        interpreter::run_with_std_stream_error_handling(
            address_copy,
            interpreter_event_sender,
            page_event_receiver,
        );
    });

    match interpreter_event_receiver.recv() {
        Ok(InterpreterEvent::SetPage(page)) => {
            let result = Window::run(Settings {
                default_font: Some("Noto".as_bytes()),
                antialiasing: true,
                text_multithreading: true,
                flags: (
                    address,
                    Some((page, page_event_sender, interpreter_event_receiver)),
                ),
                id: None,
                window: iced::window::Settings::default(),
                default_text_size: 20.,
                exit_on_close_request: true,
                try_opengles_first: false,
            });

            return match result {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("Failure running window: {e}");
                    ExitCode::FAILURE
                }
            };
        }
        Ok(InterpreterEvent::ScriptError(_)) => {
            return ExitCode::FAILURE;
        }
        Ok(InterpreterEvent::LoadAddressError(error)) => {
            eprint!("{}", error);
            return ExitCode::FAILURE;
        }
        _ => (),
    }

    ExitCode::SUCCESS
}
