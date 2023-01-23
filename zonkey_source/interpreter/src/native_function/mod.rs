use self::cli_api::{CliFunctionInteger, CliFunctionNone, CliFunctionString};

pub mod cli_api;

#[derive(Debug)]
pub enum NativeFunctionNone {
    Cli(CliFunctionNone),
}

#[derive(Debug)]
pub enum NativeFunctionString {
    Cli(CliFunctionString),
}

#[derive(Debug)]
pub enum NativeFunctionInteger {
    Cli(CliFunctionInteger),
}
