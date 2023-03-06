use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Message {
    AddressChanged(String),
    AddressConfirmed,
    SettingsPressed,
    HomePressed,
    Event(interpreter::event::Event),
    Ready(mpsc::Sender<String>),
    BootComplete(mpsc::Sender<String>),
    PageButtonPressed,
    Hyperlink(String),
}
