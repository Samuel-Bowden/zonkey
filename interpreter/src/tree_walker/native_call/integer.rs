use crate::{standard_prelude::calls::NativeCallInteger, tree_walker::object::NativeObject};

use super::prelude::*;

impl<'a> TreeWalker<'a> {
    pub fn native_call_integer(&mut self, call: &NativeCallInteger) -> Result<i64, TreeWalkerErr> {
        match call {
            NativeCallInteger::IntegerArrayGet(array, index) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)?;

                let array = array_obj.extract_native_object().extract_integer_array().lock().unwrap();

                if let Some(element) = array.get(index as usize) {
                    Ok(*element)
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange)
                }
            }

            NativeCallInteger::IntegerArrayRemove(array, index) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let mut array = array_obj.extract_native_object().extract_integer_array().lock().unwrap();

                if index < array.len() {
                    Ok(array.remove(index))
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange)
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
        }
    }
}
