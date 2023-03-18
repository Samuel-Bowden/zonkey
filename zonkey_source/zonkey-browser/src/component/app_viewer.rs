use crate::{message::Message, zonkey_app::element::ElementType};
use iced::{
    widget::{Button, Column, Scrollable, Text, TextInput},
    Alignment, Element, Length,
};

pub fn build(element_type: &ElementType) -> Element<Message> {
    match element_type {
        ElementType::Heading(val) => Column::new()
            .push(Text::new(val).size(40))
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into(),
        ElementType::Paragraph(val) => Text::new(val).into(),
        ElementType::Page(elements) => {
            let mut page_content = vec![];

            for element in elements {
                page_content.push(build(element));
            }

            Scrollable::new(Column::with_children(page_content).padding(30).spacing(20)).into()
        }
        ElementType::Button(id, text) => Button::new(Text::new(text))
            .on_press(Message::PageButtonPressed(*id))
            .into(),
        ElementType::Hyperlink(text, link) => Button::new(Text::new(text))
            .on_press(Message::Hyperlink(link.to_string()))
            .into(),
        ElementType::Input(id, placeholder, text) => {
            TextInput::new(placeholder, text, |new_value| -> Message {
                Message::InputChanged(new_value, *id)
            })
            .on_submit(Message::InputSubmit(*id))
            .into()
        }
    }
}
