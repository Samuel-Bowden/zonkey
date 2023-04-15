use interpreter::{event::InterpreterEvent, Address};
use std::sync::mpsc::Receiver;

pub type SubscriptionState = (usize, SubscriptionStateVariant);

pub enum SubscriptionStateVariant {
    PreparingForNextScript,
    ReadyForNextScript(Receiver<Address>),
    RunningScript(Receiver<InterpreterEvent>),
    Finished,
}
