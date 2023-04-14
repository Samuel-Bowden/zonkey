use crate::standard_prelude::calls::NativeCallFloat;
use super::prelude::*;

impl<'a> TreeWalker<'a> {
    pub fn native_call_float(&mut self, call: &NativeCallFloat) -> Result<f64, TreeWalkerErr> {
        match call {
            NativeCallFloat::FloatArrayGet(array, index) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)?;

                let array = array_obj.extract_native_object().extract_float_array().lock().unwrap();

                if let Some(element) = array.get(index as usize) {
                    Ok(element.clone())
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange)
                }
            }

            NativeCallFloat::FloatArrayRemove(array, index) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let mut array = array_obj.extract_native_object().extract_float_array().lock().unwrap();

                if index < array.len() {
                    Ok(array.remove(index))
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange)
                }
            }
        }
    }
}
