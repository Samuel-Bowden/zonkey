use super::prelude::*;
use crate::{event::PageEvent, standard_prelude::calls::NativeCallBoolean};

impl<'a> TreeWalker<'a> {
    pub fn native_call_boolean(&mut self, call: &NativeCallBoolean) -> Result<bool, TreeWalkerErr> {
        match call {
            NativeCallBoolean::WaitForEvent => {
                self.interpreter_event_sender
                    .send(InterpreterEvent::Update)
                    .ok();
                match self.page_event_receiver.recv() {
                    Ok(PageEvent::ButtonPress(button)) => {
                        button.lock().unwrap().clicked = true;
                        Ok(true)
                    }
                    Ok(PageEvent::InputConfirmed(input)) => {
                        input.lock().unwrap().confirmed = true;
                        Ok(true)
                    }
                    Err(_) => Ok(false),
                }
            }

            NativeCallBoolean::ButtonClicked(object) => {
                let mut object = self.eval_object(object)?;

                let mut button = object
                    .extract_native_object()
                    .extract_button()
                    .lock()
                    .unwrap();

                if button.clicked {
                    button.clicked = false;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }

            NativeCallBoolean::InputConfirmed(object) => {
                let mut object = self.eval_object(object)?;

                let mut input = object
                    .extract_native_object()
                    .extract_input()
                    .lock()
                    .unwrap();

                if input.confirmed {
                    input.confirmed = false;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }

            NativeCallBoolean::BooleanArrayGet(array, index, token) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let array = array_obj
                    .extract_native_object()
                    .extract_boolean_array()
                    .lock()
                    .unwrap();

                if let Some(element) = array.get(index) {
                    Ok(element.clone())
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange(
                        index,
                        array.len(),
                        token.clone(),
                    ))
                }
            }

            NativeCallBoolean::BooleanArrayRemove(array, index, token) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let mut array = array_obj
                    .extract_native_object()
                    .extract_boolean_array()
                    .lock()
                    .unwrap();

                if index < array.len() {
                    Ok(array.remove(index))
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange(
                        index,
                        array.len(),
                        token.clone(),
                    ))
                }
            }
        }
    }
}
