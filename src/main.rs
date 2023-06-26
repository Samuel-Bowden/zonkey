use crate::tab::Address;
use clap::{Args, Parser, Subcommand};
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
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the script in the command line interface, opening a window if a page is set.
    Run(RunArgs),
    /// Run the browser
    Browser(BrowserArgs),
}

#[derive(Args)]
struct RunArgs {
    ///A script to run in the command line interface
    script_address: String,

    #[cfg(target_os = "windows")]
    #[arg(short, long)]
    ///Disable the console (Windows only)
    disable_console: bool,

    ///Arguments to be passed to the script
    arguments: Vec<String>,

    #[arg(default_value_t = 1280, long)]
    ///Width of the window launched
    width: u32,

    #[arg(default_value_t = 720, long)]
    ///Height of the window launched
    height: u32,
}

#[derive(Args)]
struct BrowserArgs {
    #[arg(default_value_t = String::from("zonkey:home.zonk"))]
    ///A Zonkey formatted address to launch in the browser
    script_address: String,

    #[cfg(target_os = "windows")]
    #[arg(short, long)]
    ///Enable the console (Windows only)
    enable_console: bool,

    ///Arguments to be passed to the script
    arguments: Vec<String>,
}

pub fn main() -> ExitCode {
    let arguments = Arguments::parse();

    match arguments.command {
        Command::Run(run_args) => {
            let address = Address::new(&run_args.script_address, run_args.arguments);
            #[cfg(target_os = "windows")]
            disable_console(run_args.disable_console);
            command_line_tool(address, run_args.width, run_args.height)
        }
        Command::Browser(browser_args) => {
            let address = Address::new(&browser_args.script_address, browser_args.arguments);
            #[cfg(target_os = "windows")]
            disable_console(!browser_args.enable_console);
            browser(address)
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_console(disable: bool) {
    if disable {
        unsafe {
            winapi::um::wincon::FreeConsole();
        }
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
            eprintln!("Failed to open window for browser. Please make sure you are using a GPU that supports OpenGL 3.0+ or OpenGL ES 2.0. Error details: {e}");
            ExitCode::FAILURE
        }
    }
}

fn command_line_tool(address: Address, width: u32, height: u32) -> ExitCode {
    let (interpreter_event_sender, interpreter_event_receiver) = mpsc::channel();
    let (page_event_sender, page_event_receiver) = mpsc::channel();

    let address_copy = address.clone();

    let builder = thread::Builder::new().stack_size(interpreter::REQUIRED_STACK_SIZE);

    builder
        .spawn(move || {
            interpreter::run_with_error_messages(
                address_copy,
                interpreter_event_sender,
                page_event_receiver,
            );
        })
        .expect("Failed to spawn interpreter thread.");

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
                window: iced::window::Settings {
                    size: (width, height),
                    ..Default::default()
                },
                default_text_size: 20.,
                exit_on_close_request: true,
                try_opengles_first: false,
            });

            return match result {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("Failed to open window for script. Please make sure you are using a GPU that supports OpenGL 3.0+ or OpenGL ES 2.0. Error details: {e}");
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
