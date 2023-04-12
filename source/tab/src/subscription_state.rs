use std::sync::mpsc::Receiver;
use interpreter::{event::InterpreterEvent, Address};

pub type SubscriptionState = (usize, SubscriptionStateVariant);

pub enum SubscriptionStateVariant {
    PreparingForNextScript,
    ReadyForNextScript(Receiver<Address>),
    RunningScript(Receiver<InterpreterEvent>),
    Finished,
}
