use std::sync::mpsc::Receiver;
use ui::event::InterpreterEvent;

pub type SubscriptionState = (usize, SubscriptionStateVariant);

pub enum SubscriptionStateVariant {
    PreparingForNextScript,
    ReadyForNextScript(Receiver<String>),
    RunningScript(Receiver<InterpreterEvent>),
    Finished,
}
