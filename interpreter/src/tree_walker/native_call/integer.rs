use crate::{standard_prelude::calls::NativeCallInteger, tree_walker::object::NativeObject};

use super::prelude::*;

impl<'a> TreeWalker<'a> {
    pub fn native_call_integer(&mut self, call: &NativeCallInteger) -> Result<i64, TreeWalkerErr> {
        match call {
            NativeCallInteger::Power(base, exponent) => {
                let base = self.eval_int(base)?;
                let exponent = self.eval_int(exponent)?;

                Ok(i64::pow(base, exponent as u32))
            }
            NativeCallInteger::IntegerArrayGet(array, index, token) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let array = array_obj
                    .extract_native_object()
                    .extract_integer_array()
                    .lock()
                    .unwrap();

                if let Some(element) = array.get(index) {
                    Ok(*element)
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange(
                        index,
                        array.len(),
                        token.clone(),
                    ))
                }
            }

            NativeCallInteger::IntegerArrayRemove(array, index, token) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let mut array = array_obj
                    .extract_native_object()
                    .extract_integer_array()
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

            NativeCallInteger::ArrayLength(array) => {
                let mut array_obj = self.eval_object(&array)?;

                let array = array_obj.extract_native_object();

                Ok(match array {
                    NativeObject::IntegerArray(array) => array.lock().unwrap().len(),
                    NativeObject::FloatArray(array) => array.lock().unwrap().len(),
                    NativeObject::StringArray(array) => array.lock().unwrap().len(),
                    NativeObject::BooleanArray(array) => array.lock().unwrap().len(),
                    NativeObject::ObjectArray(array) => array.lock().unwrap().len(),
                    _ => panic!("Expected an array to get length of"),
                } as i64)
            }

            NativeCallInteger::FromString(string, token) => {
                let string = self.eval_string(string)?;

                match string.parse() {
                    Err(_) => Err(TreeWalkerErr::FailedStringToIntegerCast(token.clone())),
                    Ok(val) => Ok(val),
                }
            }

            NativeCallInteger::FromFloat(float) => Ok(self.eval_float(float)? as i64),
        }
    }
}
