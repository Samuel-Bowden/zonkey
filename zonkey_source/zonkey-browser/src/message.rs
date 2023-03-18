use interpreter::event::{BrowserEvent, InterpreterEvent};
use std::sync::mpsc::{self, Sender};

#[derive(Debug, Clone)]
pub enum Message {
    AddressChanged(String),
    AddressConfirmed,
    SettingsPressed,
    HomePressed,
    Event(InterpreterEvent),
    Ready(mpsc::Sender<String>),
    BootComplete(mpsc::Sender<String>),
    SetSender(Sender<BrowserEvent>),
    PageButtonPressed(i64),
    Hyperlink(String),
    InputChanged(String, i64),
    InputSubmit(i64),
}
