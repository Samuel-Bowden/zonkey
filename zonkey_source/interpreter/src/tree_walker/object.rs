use super::environment::Environment;
use crate::element::{Button, Column, Hyperlink, Input, Page, Row, Text};
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
