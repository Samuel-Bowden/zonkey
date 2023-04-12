use super::environment::Environment;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use crate::element::*;

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
}

impl NativeObject {
    pub fn extract_text(&mut self) -> &mut Arc<Mutex<Text>> {
        if let NativeObject::Text(text) = self {
            text
        } else {
            panic!("Attempted to extract type Text from a native object that was not that type")
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
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Native(NativeObject),
    Zonkey(Rc<RefCell<Environment>>),
}

impl Object {
    pub fn extract_zonkey_object(&mut self) -> &mut Rc<RefCell<Environment>> {
        if let Object::Zonkey(env) = self {
            env
        } else {
            panic!("Attempted to extract native object into a zonkey object")
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

