pub use self::{
    cli_api::{CliFunctionNone, CliFunctionString},
    gui_api::GuiFunctionNone,
};

pub mod cli_api;
pub mod gui_api;

#[derive(Debug)]
pub enum NativeFunctionNone {
    Cli(CliFunctionNone),
    Gui(GuiFunctionNone),
}

#[derive(Debug)]
pub enum NativeFunctionString {
    Cli(CliFunctionString),
}
