use std::io::Write;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

#[macro_export]
macro_rules! interpreter_debug {
    ($debug:expr, $message:literal) => { 
        #[cfg(debug_assertions)]
        if ($debug) {
            crate::debugger::report("INTEPRETER", $message, termcolor::Color::Yellow);
        }
    }
}

#[macro_export]
macro_rules! lexer_debug {
    ($debug:expr, $message:literal) => { 
        #[cfg(debug_assertions)]
        if ($debug) {
            crate::debugger::report("LEXER", $message, termcolor::Color::Blue);
        }
    }
}

#[macro_export]
macro_rules! parser_debug {
    ($debug:expr, $message:literal) => { 
        #[cfg(debug_assertions)]
        if ($debug) {
            crate::debugger::report("PARSER", $message, termcolor::Color::Magenta);
        }
    }
}

#[macro_export]
macro_rules! debug_information {
    ($debug:expr, $message:literal) => { 
        #[cfg(debug_assertions)]
        if ($debug) {
            println!("  {}", $message);
        }
    }
}

pub fn report(name: &str, message: &str, color: Color) {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .unwrap();

    write!(&mut stdout, "({name})").unwrap();

    stdout.reset().unwrap();

    writeln!(&mut stdout, " {message}").unwrap();
}
