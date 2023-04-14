use std::thread;
use colorsys::Rgb;
use resource_loader::Address;
use crate::{standard_prelude::calls::NativeCallObject, tree_walker::object::{Object, NativeObject}, element::*};
use super::prelude::*;

impl<'a> TreeWalker<'a> {
    pub fn native_call_object(&mut self, call: &NativeCallObject) -> Result<Object, TreeWalkerErr> {
        match call {
            NativeCallObject::PageAddElement(page_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut page_obj = self.eval_object(page_obj)?;

                {
                    let mut page = page_obj
                        .extract_native_object()
                        .extract_page()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    page.elements
                        .push((element.get_id(), Self::native_obj_to_element(element)));
                }

                Ok(page_obj)
            }

            NativeCallObject::PageRemoveElement(page_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut page_obj = self.eval_object(page_obj)?;

                {
                    let mut page = page_obj
                        .extract_native_object()
                        .extract_page()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    let pos = page
                        .elements
                        .iter()
                        .position(|&(id, _)| id == element.get_id());

                    if let Some(pos) = pos {
                        page.elements.remove(pos);
                    }
                }

                Ok(page_obj)
            }

            NativeCallObject::RowAddElement(row_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut row_obj = self.eval_object(row_obj)?;

                {
                    let mut row = row_obj
                        .extract_native_object()
                        .extract_row()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    row.elements
                        .push((element.get_id(), Self::native_obj_to_element(element)));
                }

                Ok(row_obj)
            }

            NativeCallObject::RowRemoveElement(row_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut row_obj = self.eval_object(row_obj)?;

                {
                    let mut row = row_obj
                        .extract_native_object()
                        .extract_row()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    let pos = row 
                        .elements
                        .iter()
                        .position(|&(id, _)| id == element.get_id());

                    if let Some(pos) = pos {
                        row.elements.remove(pos);
                    }
                }

                Ok(row_obj)
            }

            NativeCallObject::ColumnAddElement(column_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut column_obj = self.eval_object(column_obj)?;

                {
                    let mut column = column_obj
                        .extract_native_object()
                        .extract_column()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    column.elements
                        .push((element.get_id(), Self::native_obj_to_element(element)));
                }

                Ok(column_obj)
            }

            NativeCallObject::ColumnRemoveElement(column_obj, element) => {
                let mut element_obj = self.eval_object(element)?;
                let mut column_obj = self.eval_object(column_obj)?;

                {
                    let mut column = column_obj
                        .extract_native_object()
                        .extract_column()
                        .lock()
                        .unwrap();

                    let element = element_obj.extract_native_object();

                    let pos = column 
                        .elements
                        .iter()
                        .position(|&(id, _)| id == element.get_id());

                    if let Some(pos) = pos {
                        column.elements.remove(pos);
                    }
                }

                Ok(column_obj)
            }

            NativeCallObject::PageConstructor => {
                let page = Arc::new(Mutex::new(Page {
                    id: self.next_element_id(),
                    title: "Unnamed Application".to_string(),
                    elements: vec![],
                    bg_red: 255,
                    bg_green: 255,
                    bg_blue: 255,
                    txt_red: 0,
                    txt_green: 0,
                    txt_blue: 0,
                    center: false,
                    max_width: None,
                }));

                Ok(Object::Native(NativeObject::Page(page)))
            }

            NativeCallObject::ObjectArrayConstructor(exprs) => {
                let mut array = vec![];

                for expr in exprs {
                    array.push(self.eval_object(expr.to_object_expr_ref())?);
                }

                let array_obj = Arc::new(Mutex::new(array));

                Ok(Object::Native(NativeObject::ObjectArray(array_obj)))
            }

            NativeCallObject::ObjectArrayPush(array, element) => {
                let mut array_obj = self.eval_object(&array)?;
                let element = self.eval_object(element)?;

                array_obj 
                    .extract_native_object()
                    .extract_object_array()
                    .lock()
                    .unwrap()
                    .push(element);

                Ok(array_obj)
            }

            NativeCallObject::ObjectArrayGet(array, index) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)?;

                let array = array_obj.extract_native_object().extract_object_array().lock().unwrap();

                if let Some(element) = array.get(index as usize) {
                    Ok(element.clone())
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange)
                }
            }

            NativeCallObject::ObjectArrayRemove(array, index) => {
                let mut array_obj = self.eval_object(&array)?;
                let index = self.eval_int(index)? as usize;

                let mut array = array_obj.extract_native_object().extract_object_array().lock().unwrap();

                if index < array.len() {
                    Ok(array.remove(index))
                } else {
                    Err(TreeWalkerErr::IndexOutOfRange)
                }
            }

            NativeCallObject::IntegerArrayConstructor(exprs) => {
                let mut array = vec![];

                for expr in exprs {
                    array.push(self.eval_int(expr.to_integer_expr_ref())?);
                }

                let array_obj = Arc::new(Mutex::new(array));

                Ok(Object::Native(NativeObject::IntegerArray(array_obj)))
            }

            NativeCallObject::IntegerArrayPush(array, element) => {
                let mut array_obj = self.eval_object(&array)?;
                let element = self.eval_int(element)?;

                array_obj 
                    .extract_native_object()
                    .extract_integer_array()
                    .lock()
                    .unwrap()
                    .push(element);

                Ok(array_obj)
            }

            NativeCallObject::FloatArrayConstructor(exprs) => {
                let mut array = vec![];

                for expr in exprs {
                    array.push(self.eval_float(expr.to_float_expr_ref())?);
                }

                let array_obj = Arc::new(Mutex::new(array));

                Ok(Object::Native(NativeObject::FloatArray(array_obj)))
            }

            NativeCallObject::FloatArrayPush(array, element) => {
                let mut array_obj = self.eval_object(&array)?;
                let element = self.eval_float(element)?;

                array_obj 
                    .extract_native_object()
                    .extract_float_array()
                    .lock()
                    .unwrap()
                    .push(element);

                Ok(array_obj)
            }

            NativeCallObject::StringArrayConstructor(exprs) => {
                let mut array = vec![];

                for expr in exprs {
                    array.push(self.eval_string(expr.to_string_expr_ref())?);
                }

                let array_obj = Arc::new(Mutex::new(array));

                Ok(Object::Native(NativeObject::StringArray(array_obj)))
            }

            NativeCallObject::StringArrayPush(array, element) => {
                let mut array_obj = self.eval_object(&array)?;
                let element = self.eval_string(element)?;

                array_obj 
                    .extract_native_object()
                    .extract_string_array()
                    .lock()
                    .unwrap()
                    .push(element);

                Ok(array_obj)
            }

            NativeCallObject::BooleanArrayConstructor(exprs) => {
                let mut array = vec![];

                for expr in exprs {
                    array.push(self.eval_boolean(expr.to_boolean_expr_ref())?);
                }

                let array_obj = Arc::new(Mutex::new(array));

                Ok(Object::Native(NativeObject::BooleanArray(array_obj)))
            }

            NativeCallObject::BooleanArrayPush(array, element) => {
                let mut array_obj = self.eval_object(&array)?;
                let element = self.eval_boolean(element)?;

                array_obj 
                    .extract_native_object()
                    .extract_boolean_array()
                    .lock()
                    .unwrap()
                    .push(element);

                Ok(array_obj)
            }

            NativeCallObject::ButtonConstructor(text) => {
                let text = self.eval_string(text)?;
                let button = Arc::new(Mutex::new(Button {
                    id: self.next_element_id(),
                    text,
                    bg_red: 127,
                    bg_green: 127,
                    bg_blue: 127,
                    txt_red: 255,
                    txt_green: 255,
                    txt_blue: 255,
                    clicked: false,
                    vertical_padding: 10.,
                    horizontal_padding: 10.,
                    width_fill: false,
                }));
                Ok(Object::Native(NativeObject::Button(button)))
            }

            NativeCallObject::ButtonSetText(object, text) => {
                let mut object = self.eval_object(object)?;
                let text = self.eval_string(text)?;

                object
                    .extract_native_object()
                    .extract_button()
                    .lock()
                    .unwrap()
                    .text = text;

                Ok(object)
            }

            NativeCallObject::TextConstructor(value) => {
                let value = self.eval_string(value)?;
                let text = Arc::new(Mutex::new(Text {
                    id: self.next_element_id(),
                    size: 20.,
                    value,
                    colour: None,
                }));
                Ok(Object::Native(NativeObject::Text(text)))
            }

            NativeCallObject::TextSetValue(text, value) => {
                let mut object = self.eval_object(text)?;
                let value = self.eval_string(value)?;

                object
                    .extract_native_object()
                    .extract_text()
                    .lock()
                    .unwrap()
                    .value = value;

                Ok(object)
            }

            NativeCallObject::HyperlinkConstructor(text, link) => {
                let text = self.eval_string(text)?;
                let link = self.eval_string(link)?;
                let hyperlink = Arc::new(Mutex::new(Hyperlink {
                    id: self.next_element_id(),
                    link,
                    text,
                }));
                Ok(Object::Native(NativeObject::Hyperlink(hyperlink)))
            }

            NativeCallObject::InputConstructor(placeholder) => {
                let placeholder = self.eval_string(placeholder)?;
                let input = Arc::new(Mutex::new(Input {
                    id: self.next_element_id(),
                    placeholder,
                    text: String::new(),
                    confirmed: false,
                }));
                Ok(Object::Native(NativeObject::Input(input)))
            }

            NativeCallObject::ImageConstructor(link) => {
                let link = self.eval_string(link)?;
                let image = Arc::new(Mutex::new(Image {
                    data: None,
                    id: self.next_element_id(),
                    max_width: None,
                }));

                let image_ref = Arc::clone(&image);

                let sender_clone = self.interpreter_event_sender.clone();

                thread::spawn(move || {
                    let data = Address::new(&link).load_image();
                    image_ref.lock().unwrap().data = Some(data);
                    sender_clone.send(InterpreterEvent::Update).unwrap();
                });

                Ok(Object::Native(NativeObject::Image(image)))
            }

            NativeCallObject::TextSetSize(object, size) => {
                let mut object = self.eval_object(object)?;
                let size = self.eval_float(size)?;

                object
                    .extract_native_object()
                    .extract_text()
                    .lock()
                    .unwrap()
                    .size = size as f32;

                Ok(object)
            }

            NativeCallObject::TextSetColour(object, hex) => {
                let mut object = self.eval_object(object)?;
                let rgb = Self::colour(self.eval_string(hex)?)?;

                {
                    let mut text = object
                        .extract_native_object()
                        .extract_text()
                        .lock()
                        .unwrap();
                    text.colour = Some((rgb.red() as u8, rgb.green() as u8, rgb.blue() as u8))
                }

                Ok(object)
            }

            NativeCallObject::ButtonSetBackgroundColour(object, hex) => {
                let mut object = self.eval_object(object)?;
                let rgb = Self::colour(self.eval_string(hex)?)?;

                {
                    let mut button = object
                        .extract_native_object()
                        .extract_button()
                        .lock()
                        .unwrap();

                    button.bg_red = rgb.red() as u8;
                    button.bg_green = rgb.green() as u8;
                    button.bg_blue = rgb.blue() as u8;
                }

                Ok(object)
            }

            NativeCallObject::ButtonSetTextColour(object, hex) => {
                let mut object = self.eval_object(object)?;
                let rgb = Self::colour(self.eval_string(hex)?)?;

                {
                    let mut button = object
                        .extract_native_object()
                        .extract_button()
                        .lock()
                        .unwrap();

                    button.txt_red = rgb.red() as u8;
                    button.txt_green = rgb.green() as u8;
                    button.txt_blue = rgb.blue() as u8;
                }

                Ok(object)
            }

            NativeCallObject::ButtonSetPadding(object, vertical, horizontal) => {
                let mut object = self.eval_object(object)?;
                let vertical = self.eval_float(vertical)?;
                let horizontal = self.eval_float(horizontal)?;

                {
                    let mut button = object
                        .extract_native_object()
                        .extract_button()
                        .lock()
                        .unwrap();

                    button.vertical_padding = vertical as f32;
                    button.horizontal_padding = horizontal as f32;
                }

                Ok(object)
            }

            NativeCallObject::PageSetTitle(page, title) => {
                let mut object = self.eval_object(page)?;
                let title = self.eval_string(title)?;

                object
                    .extract_native_object()
                    .extract_page()
                    .lock()
                    .unwrap()
                    .title = title;

                Ok(object)
            }

            NativeCallObject::PageSetBackgroundColour(page, hex) => {
                let mut object = self.eval_object(page)?;
                let rgb = Self::colour(self.eval_string(hex)?)?;

                {
                    let mut page = object
                        .extract_native_object()
                        .extract_page()
                        .lock()
                        .unwrap();

                    page.bg_red = rgb.red() as u8;
                    page.bg_green = rgb.green() as u8;
                    page.bg_blue = rgb.blue() as u8;
                }

                Ok(object)
            }

            NativeCallObject::PageSetTextColour(page, hex) => {
                let mut object = self.eval_object(page)?;
                let rgb = Self::colour(self.eval_string(hex)?)?;

                {
                    let mut page = object
                        .extract_native_object()
                        .extract_page()
                        .lock()
                        .unwrap();

                    page.txt_red = rgb.red() as u8;
                    page.txt_green = rgb.green() as u8;
                    page.txt_blue = rgb.blue() as u8;
                }

                Ok(object)
            }

            NativeCallObject::RowConstructor => {
                let row = Arc::new(Mutex::new(Row {
                    id: self.next_element_id(),
                    elements: vec![],
                    center: false,
                }));
                Ok(Object::Native(NativeObject::Row(row)))
            }

            NativeCallObject::ColumnConstructor => {
                let column = Arc::new(Mutex::new(Column {
                    id: self.next_element_id(),
                    elements: vec![],
                    max_width: None,
                }));
                Ok(Object::Native(NativeObject::Column(column)))
            }

            NativeCallObject::ButtonSetWidthFill(obj) => {
                let mut button = self.eval_object(obj)?;

                button
                    .extract_native_object()
                    .extract_button()
                    .lock()
                    .unwrap()
                    .width_fill = true;

                Ok(button)
            }

            NativeCallObject::ColumnSetMaxWidth(obj, width) => {
                let mut button = self.eval_object(obj)?;
                let width = self.eval_float(width)?;

                button
                    .extract_native_object()
                    .extract_column()
                    .lock()
                    .unwrap()
                    .max_width = Some(width as f32);

                Ok(button)
            }

            NativeCallObject::RowCenter(obj) => {
                let mut row = self.eval_object(obj)?;

                row.extract_native_object()
                    .extract_row()
                    .lock()
                    .unwrap()
                    .center = true;

                Ok(row)
            }

            NativeCallObject::ImageSetMaxWidth(obj, width) => {
                let mut image = self.eval_object(obj)?;
                let width = self.eval_float(width)?;

                image
                    .extract_native_object()
                    .extract_image()
                    .lock()
                    .unwrap()
                    .max_width = Some(width as f32);

                Ok(image)
            }

            NativeCallObject::PageCenter(page) => {
                let mut page = self.eval_object(page)?;

                page 
                    .extract_native_object()
                    .extract_page()
                    .lock()
                    .unwrap()
                    .center = true;

                Ok(page)
            }

            NativeCallObject::PageSetMaxWidth(page, max_width) => {
                let mut page = self.eval_object(page)?;
                let max_width = self.eval_float(max_width)?;

                page 
                    .extract_native_object()
                    .extract_page()
                    .lock()
                    .unwrap()
                    .max_width = Some(max_width as f32);

                Ok(page)
            }

            NativeCallObject::InputSetText(input, text) => {
                let mut input = self.eval_object(input)?;
                let text = self.eval_string(text)?;

                input 
                    .extract_native_object()
                    .extract_input()
                    .lock()
                    .unwrap()
                    .text = text;

                Ok(input)
            }
        }
    }

    fn colour(hex: String) -> Result<Rgb, TreeWalkerErr> {
        match Rgb::from_hex_str(hex.as_str()) {
            Ok(rgb) => Ok(rgb),
            Err(_) => Err(TreeWalkerErr::InvalidHexColour(hex)),
        }
    }
}
