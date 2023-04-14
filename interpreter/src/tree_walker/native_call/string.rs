use std::io::{stdout, Write};
use resource_loader::Address;

use crate::standard_prelude::calls::NativeCallString;
use super::prelude::*;

impl<'a> TreeWalker<'a> {
    pub fn native_call_string(&mut self, call: &NativeCallString) -> Result<String, TreeWalkerErr> {
        match call {
            NativeCallString::Prompt(expr) => {
                let prompt = self.eval_string(expr)?;

                self.stdout.extend_from_slice(prompt.as_bytes());
                self.stdout.extend_from_slice(" ".as_bytes());
                stdout().write_all(&self.stdout.as_slice()).unwrap();
                stdout().flush().unwrap();
                self.stdout.clear();

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();

                Ok(input.trim().to_string())
            }
            NativeCallString::GetInputText(input) => {
                let mut input = self.eval_object(input)?;

                let text = input
                    .extract_native_object()
                    .extract_input()
                    .lock()
                    .unwrap()
                    .text
                    .clone();

                Ok(text)
            }

            NativeCallString::ReadString(location) => {
                let location = self.eval_string(location)?;

                let string = match Address::new(&location).read_string() {
                    Ok(string) => string,
                    Err(e) => return Err(TreeWalkerErr::ReadAddressFailed(e.to_string())),
                };

                Ok(string)
            }

            NativeCallString::StringArrayGet(array, index) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)?;

                let array = array_obj.extract_native_object().extract_string_array().lock().unwrap();

                if let Some(element) = array.get(index as usize) {
                    Ok(element.clone())
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange)
                }
            }

            NativeCallString::StringArrayRemove(array, index) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let mut array = array_obj.extract_native_object().extract_string_array().lock().unwrap();

                if index < array.len() {
                    Ok(array.remove(index))
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange)
                }
            }

            NativeCallString::FromInteger(integer) => {
                Ok(self.eval_int(integer)?.to_string())
            }

            NativeCallString::FromFloat(float) => {
                Ok(self.eval_float(float)?.to_string())
            }
        }
    }
}
