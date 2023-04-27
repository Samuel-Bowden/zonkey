use crate::tab::Address;
use clap::Parser;
use interpreter::{
    event::InterpreterEvent,
    iced::{self, Application, Settings},
};
use std::{process::ExitCode, sync::mpsc, thread};
use window::Window;

mod tab;
mod window;

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
struct Arguments {
    #[arg(verbatim_doc_comment)]
    ///A Zonkey formatted address to load the script from, e.g.
    ///- 'scripts/hello_world.zonk'
    ///- 'zonkey:home.zonk'
    ///- 'https://twigville.com/app.zonk'
    script_address: String,

    #[arg(short, long, verbatim_doc_comment)]
    ///Launch the browser mode:
    ///Provide the address 'zonkey:home.zonk' for the home page
    browser: bool,

    #[arg(short, long, raw = true)]
    ///Arguments to be passed to the script
    arguments: Vec<String>,

    #[cfg(target_os = "windows")]
    #[arg(short, long, verbatim_doc_comment)]
    ///Disables the console (Windows only)
    disable_console: bool,
}

pub fn main() -> ExitCode {
    let arguments = Arguments::parse();
    let address = Address::new(&arguments.script_address, arguments.arguments);

    #[cfg(target_os = "windows")]
    if arguments.disable_console {
        unsafe {
            winapi::um::wincon::FreeConsole();
        }
    }

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
        interpreter::run_with_error_messages(
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
