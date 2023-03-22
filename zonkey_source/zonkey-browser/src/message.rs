use interpreter::{
    element::{Button, Input},
    event::{BrowserEvent, InterpreterEvent},
};
use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};

#[derive(Debug, Clone)]
pub enum Message {
    AddressChanged(String),
    AddressConfirmed,
    SettingsPressed,
    HomePressed,
    ReloadPressed,
    BackPressed,
    Event(InterpreterEvent),
    Ready(mpsc::Sender<String>),
    BootComplete(mpsc::Sender<String>),
    SetSender(Sender<BrowserEvent>),
    PageButtonPressed(Arc<Mutex<Button>>),
    Hyperlink(String),
    InputChanged(String, Arc<Mutex<Input>>),
    InputSubmit(Arc<Mutex<Input>>),
}
