use iced::{Application, Settings};
use resource_loader::Address;
use std::{env::args, process::ExitCode, sync::mpsc, thread};
use ui::event::InterpreterEvent;
use window::Window;

mod window;

fn main() -> ExitCode {
    if let Some(value) = args().skip(1).next() {
        let address = match Address::new(&value) {
            Ok(address) => address,
            Err(err) => {
                eprintln!("Improperly formatted address: {err}");
                return ExitCode::FAILURE;
            }
        };

        run(address)
    } else {
        eprintln!("Error: Missing address argument\nUsage: zonkey <ADDRESS>");
        ExitCode::FAILURE
    }
}

fn run(address: Address) -> ExitCode {
    let source = match address.load_script() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Couldn't open address ({e})");
            // Exit code for non-existent or unreadable address
            return ExitCode::from(66);
        }
    };

    let (interpreter_sender, this_receiver) = mpsc::channel();
    let (this_sender, interpreter_receiver) = mpsc::channel();

    thread::spawn(move || {
        interpreter::run_with_std_stream_error_handling(
            source,
            interpreter_sender,
            interpreter_receiver,
        );
    });

    if let Ok(InterpreterEvent::NewPage(page)) = this_receiver.recv() {
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

    ExitCode::SUCCESS
}
