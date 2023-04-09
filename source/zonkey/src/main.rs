use iced::{Application, Settings};
use resource_loader::Address;
use std::{env::args, process::ExitCode, sync::mpsc, thread};
use ui::event::InterpreterEvent;
use window::Window;

mod window;

fn main() -> ExitCode {
    if let Some(value) = args().skip(1).next() {
        let address = match Address::new(&value) {
            Address::Invalid(_, err) => {
                eprintln!("Improperly formatted address: {err}");
                return ExitCode::FAILURE;
            }
            address => address,
        };

        run(address)
    } else {
        eprintln!("Error: Missing address argument\nUsage: zonkey <ADDRESS>");
        ExitCode::FAILURE
    }
}

fn run(address: Address) -> ExitCode {
    let (interpreter_sender, this_receiver) = mpsc::channel();
    let (this_sender, interpreter_receiver) = mpsc::channel();

    let address_copy = address.clone();

    thread::spawn(move || {
        interpreter::run_with_std_stream_error_handling(
            address_copy,
            interpreter_sender,
            interpreter_receiver,
        );
    });

    match this_receiver.recv() {
        Ok(InterpreterEvent::NewPage(page)) => {
            let result = Window::run(Settings {
                default_font: Some("Noto".as_bytes()),
                antialiasing: true,
                text_multithreading: true,
                flags: (page, this_sender, this_receiver, address),
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
        Ok(InterpreterEvent::ScriptError(error) | InterpreterEvent::LoadAddressError(error)) => {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
        _ => (),
    }

    ExitCode::SUCCESS
}
