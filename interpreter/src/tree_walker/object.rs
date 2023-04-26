use super::state::State;
use crate::element::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub enum NativeObject {
    Page(Arc<Mutex<Page>>),
    Button(Arc<Mutex<Button>>),
    Text(Arc<Mutex<Text>>),
    Hyperlink(Arc<Mutex<Hyperlink>>),
    Input(Arc<Mutex<Input>>),
    Row(Arc<Mutex<Row>>),
    Column(Arc<Mutex<Column>>),
    Image(Arc<Mutex<Image>>),
    IntegerArray(Arc<Mutex<Vec<i64>>>),
    FloatArray(Arc<Mutex<Vec<f64>>>),
    StringArray(Arc<Mutex<Vec<String>>>),
    BooleanArray(Arc<Mutex<Vec<bool>>>),
    ObjectArray(Arc<Mutex<Vec<Object>>>),
}

impl NativeObject {
    pub fn extract_text(&mut self) -> &mut Arc<Mutex<Text>> {
        if let NativeObject::Text(text) = self {
            text
        } else {
            panic!("Attempted to extract type Text from a native object that was not that type")
        }
    }

    pub fn extract_hyperlink(&mut self) -> &mut Arc<Mutex<Hyperlink>> {
        if let NativeObject::Hyperlink(hyperlink) = self {
            hyperlink
        } else {
            panic!(
                "Attempted to extract type Hyperlink from a native object that was not that type"
            )
        }
    }

    pub fn extract_row(&mut self) -> &mut Arc<Mutex<Row>> {
        if let NativeObject::Row(text) = self {
            text
        } else {
            panic!("Attempted to extract type Row from a native object that was not that type")
        }
    }

    pub fn extract_column(&mut self) -> &mut Arc<Mutex<Column>> {
        if let NativeObject::Column(column) = self {
            column
        } else {
            panic!("Attempted to extract type Column from a native object that was not that type")
        }
    }

    pub fn extract_page(&mut self) -> &mut Arc<Mutex<Page>> {
        if let NativeObject::Page(page) = self {
            page
        } else {
            panic!("Attempted to extract type Page from a native object that was not that type")
        }
    }

    pub fn extract_button(&mut self) -> &mut Arc<Mutex<Button>> {
        if let NativeObject::Button(button) = self {
            button
        } else {
            panic!("Attempted to extract type Button from a native object that was not that type")
        }
    }

    pub fn extract_input(&mut self) -> &mut Arc<Mutex<Input>> {
        if let NativeObject::Input(input) = self {
            input
        } else {
            panic!("Attempted to extract type Input from a native object that was not that type")
        }
    }

    pub fn extract_image(&mut self) -> &mut Arc<Mutex<Image>> {
        if let NativeObject::Image(image) = self {
            image
        } else {
            panic!("Attempted to extract type Image from a native object that was not that type")
        }
    }

    pub fn extract_integer_array(&mut self) -> &mut Arc<Mutex<Vec<i64>>> {
        if let NativeObject::IntegerArray(array) = self {
            array
        } else {
            panic!(
                "Attempted to extract type [Integer] from a native object that was not that type"
            )
        }
    }

    pub fn extract_float_array(&mut self) -> &mut Arc<Mutex<Vec<f64>>> {
        if let NativeObject::FloatArray(array) = self {
            array
        } else {
            panic!("Attempted to extract type [Float] from a native object that was not that type")
        }
    }

    pub fn extract_string_array(&mut self) -> &mut Arc<Mutex<Vec<String>>> {
        if let NativeObject::StringArray(array) = self {
            array
        } else {
            panic!(
                "Attempted to extract type [String] from a native object that was {:?}",
                self
            )
        }
    }

    pub fn extract_boolean_array(&mut self) -> &mut Arc<Mutex<Vec<bool>>> {
        if let NativeObject::BooleanArray(array) = self {
            array
        } else {
            panic!(
                "Attempted to extract type [Boolean] from a native object that was not that type"
            )
        }
    }

    pub fn extract_object_array(&mut self) -> &mut Arc<Mutex<Vec<Object>>> {
        if let NativeObject::ObjectArray(array) = self {
            array
        } else {
            panic!("Attempted to extract type [Object] from a native object that was not that type")
        }
    }

    pub fn get_id(&mut self) -> u64 {
        match self {
            Self::Page(obj) => obj.lock().unwrap().id,
            Self::Button(obj) => obj.lock().unwrap().id,
            Self::Text(obj) => obj.lock().unwrap().id,
            Self::Hyperlink(obj) => obj.lock().unwrap().id,
            Self::Input(obj) => obj.lock().unwrap().id,
            Self::Row(obj) => obj.lock().unwrap().id,
            Self::Column(obj) => obj.lock().unwrap().id,
            Self::Image(obj) => obj.lock().unwrap().id,
            _ => unreachable!("Not applicable for this object"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Native(NativeObject),
    Zonkey(Rc<RefCell<State>>),
}

impl Object {
    pub fn extract_zonkey_object(&mut self) -> &mut Rc<RefCell<State>> {
        if let Object::Zonkey(env) = self {
            env
        } else {
            panic!(
                "Attempted to extract native object into a zonkey object, {:?}",
                self
            )
        }
    }

    pub fn extract_native_object(&mut self) -> &mut NativeObject {
        if let Object::Native(obj) = self {
            obj
        } else {
            panic!("Attempted to extract native object into a zonkey object")
        }
    }
}
