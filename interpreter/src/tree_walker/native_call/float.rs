use super::prelude::*;
use crate::standard_prelude::calls::NativeCallFloat;

impl<'a> TreeWalker<'a> {
    pub fn native_call_float(&mut self, call: &NativeCallFloat) -> Result<f64, TreeWalkerErr> {
        match call {
            NativeCallFloat::FloatArrayGet(array, index, token) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let array = array_obj
                    .extract_native_object()
                    .extract_float_array()
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

            NativeCallFloat::FloatArrayRemove(array, index, token) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let mut array = array_obj
                    .extract_native_object()
                    .extract_float_array()
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

            NativeCallFloat::FromString(string, token) => {
                let string = self.eval_string(string)?;

                match string.parse() {
                    Err(_) => Err(TreeWalkerErr::FailedStringToFloatCast(token.clone())),
                    Ok(val) => Ok(val),
                }
            }

            NativeCallFloat::FromInteger(integer) => Ok(self.eval_int(integer)? as f64),
        }
    }
}
