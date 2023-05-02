#[macro_export]
macro_rules! interpreter_debug {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        crate::debugger::report("INTEPRETER", $message);
    };
}

#[macro_export]
macro_rules! lexer_debug {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        crate::debugger::report("LEXER", $message);
    };
}

#[macro_export]
macro_rules! parser_debug {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        crate::debugger::report("PARSER", $message);
    };
}

#[macro_export]
macro_rules! tree_walker_debug {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        crate::debugger::report("TREE WALKER", $message);
    };
}

#[macro_export]
macro_rules! debug_information {
    ($message:expr) => {
        #[cfg(debug_assertions)]
        println!("  {}", $message);
    };
}

#[cfg(debug_assertions)]
pub fn report(name: &str, message: &str) {
    println!("({name}) {message}");
}
