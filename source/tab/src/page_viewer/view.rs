use super::{message::Message, PageViewer};
use iced::{
    widget::{text, Button, Column, Container, Image, Row, Scrollable, Space, Text, TextInput},
    Color, Element, Length, Padding,
};
use iced_native::theme;
use resource_loader::Address;
use std::sync::{Arc, Mutex};
use ui::element::{self, ElementType};

impl PageViewer {
    pub fn view(&self) -> Element<Message> {
        if let Some(page) = &self.page {
            Container::new(build_page(page))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Container::Custom(Box::new(
                    page.lock().unwrap().clone(),
                )))
                .into()
        } else {
            text("Loading").into()
        }
    }
}

fn build_calls<'a>(element: &ElementType) -> Element<'a, Message> {
    match element {
        ElementType::Text(text) => build_text(text.clone()),
        ElementType::Input(input) => build_input(input.clone()),
        ElementType::Hyperlink(hyperlink) => build_hyperlink(hyperlink.clone()),
        ElementType::Button(button) => build_button(button.clone()),
        ElementType::Row(row) => build_row(row.clone()),
        ElementType::Column(column) => build_column(column.clone()),
        ElementType::Image(img) => build_image(img.clone()),
    }
}

pub fn build_page<'a>(page: &Arc<Mutex<element::Page>>) -> Element<'a, Message> {
    let page = page.lock().unwrap();
    let mut page_content = vec![];

    for (_, element) in &page.elements {
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
        .on_press(Message::ButtonPressed(obj.clone()))
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
        .on_press(Message::HyperlinkPressed(hyperlink.link.to_string()))
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

fn build_image<'a>(image: Arc<Mutex<element::Image>>) -> Element<'a, Message> {
    let image_obj = image.lock().unwrap();

    match Address::new(&image_obj.link) {
        Ok(address) => {
            let image = Image::new(address.get_path());

            if let Some(max_width) = image_obj.max_width {
                image.width(max_width).into()
            } else {
                image.into()
            }
        }
        Err(_) => text("Image has an invalid address").into(),
    }
}
