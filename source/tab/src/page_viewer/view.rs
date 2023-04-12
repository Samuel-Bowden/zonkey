use super::{message::Message, PageErr, PageViewer};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{text, Button, Column, Container, Image, Row, Scrollable, Space, Text, TextInput},
    Color, Element, Length, Padding,
};
use iced_native::{image::Handle, theme};
use std::sync::{Arc, Mutex};
use interpreter::element::{self, ElementType};

impl PageViewer {
    pub fn view(&self) -> Element<Message> {
        if let Some(error) = &self.page_error {
            return match error {
                PageErr::ScriptError(error) => script_error_page(error),
                PageErr::LoadAddressError(error) => load_address_error_page(error),
            };
        }

        if let Some(page) = &self.page {
            build_page(page)
        } else {
            Container::new(text("Loading page").size(40))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
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

    let mut column = Column::with_children(page_content)
            .padding(30)
            .spacing(20);

    if let Some(max_width) = page.max_width {
        column = column.max_width(max_width);
    }

    let scrollable = Scrollable::new(column);

    let mut container = Container::new(scrollable)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(
            page.clone(),
        )));

    if page.center {
        container = container.center_x();
    }

    container.into()
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

    for (_, element) in &row.elements {
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
    let column_obj = column.lock().unwrap();
    let mut column_content = vec![];

    for (_, element) in &column_obj.elements {
        column_content.push(build_calls(element));
    }

    let mut column = Column::with_children(column_content)
        .spacing(10);

    if let Some(max_width) = column_obj.max_width {
        column = column.max_width(max_width);
    }

    column.into()
}

fn build_image<'a>(image: Arc<Mutex<element::Image>>) -> Element<'a, Message> {
    let image_obj = image.lock().unwrap();

    let image = if let Some(data) = &image_obj.data {
        Image::new(Handle::from_memory(data.clone()))
    } else {
        return text("Loading image").into();
    };

    if let Some(max_width) = image_obj.max_width {
        image.width(max_width).into()
    } else {
        image.into()
    }
}

fn script_error_page<'a>(error: &str) -> Element<'a, Message> {
    Column::new()
        .push(text("Failed to run application").size(100))
        .push(text("Execution of the script failed:"))
        .push(text(error))
        .padding(20)
        .spacing(20)
        .into()
}

fn load_address_error_page<'a>(error: &str) -> Element<'a, Message> {
    Column::new()
        .push(text("Failed to load application").size(100))
        .push(text(error))
        .padding(20)
        .spacing(20)
        .into()
}
