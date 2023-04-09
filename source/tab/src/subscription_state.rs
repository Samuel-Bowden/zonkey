use resource_loader::Address;
use std::sync::mpsc::Receiver;
use ui::event::InterpreterEvent;

pub type SubscriptionState = (usize, SubscriptionStateVariant);

pub enum SubscriptionStateVariant {
    PreparingForNextScript,
    ReadyForNextScript(Receiver<Address>),
    RunningScript(Receiver<InterpreterEvent>),
    Finished,
}
