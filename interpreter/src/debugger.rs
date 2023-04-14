#[cfg(debug_assertions)]
use termcolor::Color;

#[macro_export]
macro_rules! interpreter_debug {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        crate::debugger::report("INTEPRETER", $message, termcolor::Color::Yellow);
    };
}

#[macro_export]
macro_rules! lexer_debug {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        crate::debugger::report("LEXER", $message, termcolor::Color::Blue);
    };
}

#[macro_export]
macro_rules! parser_debug {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        crate::debugger::report("PARSER", $message, termcolor::Color::Magenta);
    };
}

#[macro_export]
macro_rules! tree_walker_debug {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        crate::debugger::report("TREE WALKER", $message, termcolor::Color::Green);
    };
}

#[macro_export]
macro_rules! debug_information {
    ($message:literal) => {
        #[cfg(debug_assertions)]
        println!("  {}", $message);
    };
}

#[cfg(debug_assertions)]
pub fn report(name: &str, message: &str, color: Color) {
    use std::io::Write;
    use termcolor::{ColorSpec, StandardStream, WriteColor};

    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .unwrap();

    write!(&mut stdout, "({name})").unwrap();

    stdout.reset().unwrap();

    writeln!(&mut stdout, " {message}").unwrap();
}
