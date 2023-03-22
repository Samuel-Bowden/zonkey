use crate::message::Message;
use iced::{
    widget::{Button, Column, Row, Scrollable, Space, Text, TextInput},
    Color, Element, Length, Padding,
};
use iced_native::theme;
use interpreter::element::{self, ElementType};
use std::sync::{Arc, Mutex};

fn build_calls<'a>(element: &ElementType) -> Element<'a, Message> {
    match element {
        ElementType::Text(text) => build_text(text.clone()),
        ElementType::Input(input) => build_input(input.clone()),
        ElementType::Hyperlink(hyperlink) => build_hyperlink(hyperlink.clone()),
        ElementType::Button(button) => build_button(button.clone()),
        ElementType::Row(row) => build_row(row.clone()),
        ElementType::Column(column) => build_column(column.clone()),
    }
}

pub fn build_page<'a>(page: Arc<Mutex<element::Page>>) -> Element<'a, Message> {
    let page = page.lock().unwrap();
    let mut page_content = vec![];

    for element in &page.elements {
        page_content.push(build_calls(element));
    }

    Scrollable::new(
        Column::with_children(page_content)
            .padding(30)
            .spacing(20)
            .width(Length::Fill),
    )
    .into()
}

fn build_text<'a>(text: Arc<Mutex<element::Text>>) -> Element<'a, Message> {
    let text = text.lock().unwrap();
    Text::new(text.value.clone())
        .size(text.size)
        .style(Color::from([text.red, text.green, text.blue]))
        .into()
}

fn build_button<'a>(obj: Arc<Mutex<element::Button>>) -> Element<'a, Message> {
    let button = obj.lock().unwrap();
    let button_ui = Button::new(Text::new(button.text.clone()))
        .on_press(Message::PageButtonPressed(obj.clone()))
        .style(theme::Button::Custom(Box::new(button.clone())))
        .padding(Padding::from([
            button.vertical_padding,
            button.horizontal_padding,
        ]));

    if button.width_fill {
        button_ui.width(Length::Fill).into()
    } else {
        button_ui.into()
    }
}

fn build_hyperlink<'a>(hyperlink: Arc<Mutex<element::Hyperlink>>) -> Element<'a, Message> {
    let hyperlink = hyperlink.lock().unwrap();
    Button::new(Text::new(hyperlink.text.clone()))
        .on_press(Message::Hyperlink(hyperlink.link.to_string()))
        .style(theme::Button::Custom(Box::new(hyperlink.clone())))
        .padding(Padding::new(0.))
        .into()
}

fn build_input<'a>(obj: Arc<Mutex<element::Input>>) -> Element<'a, Message> {
    let obj_ref = Arc::clone(&obj);
    let input = obj_ref.lock().unwrap();
    TextInput::new(
        input.placeholder.to_string().as_str(),
        input.text.to_string().as_str(),
        move |new_value| -> Message { Message::InputChanged(new_value, Arc::clone(&obj)) },
    )
    .on_submit(Message::InputSubmit(Arc::clone(&obj_ref)))
    .into()
}

fn build_row<'a>(row: Arc<Mutex<element::Row>>) -> Element<'a, Message> {
    let row = row.lock().unwrap();
    let mut row_content = vec![];

    if row.center {
        row_content.push(Space::with_width(Length::Fill).into());
    }

    for element in &row.elements {
        row_content.push(build_calls(element));
    }

    if row.center {
        row_content.push(Space::with_width(Length::Fill).into());
    }

    Row::with_children(row_content)
        .spacing(10)
        .width(Length::Fill)
        .into()
}

fn build_column<'a>(column: Arc<Mutex<element::Column>>) -> Element<'a, Message> {
    let column = column.lock().unwrap();
    let mut column_content = vec![];

    for element in &column.elements {
        column_content.push(build_calls(element));
    }

    Column::with_children(column_content)
        .spacing(10)
        .max_width(column.max_width)
        .into()
}
