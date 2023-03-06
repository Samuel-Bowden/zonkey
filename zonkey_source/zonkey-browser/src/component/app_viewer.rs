use crate::{message::Message, zonkey_app::element::ElementType};
use iced::{
    widget::{Button, Column, Image, Scrollable, Text},
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
                page_content.push(build(&element.element_type));
            }

            Scrollable::new(Column::with_children(page_content).padding(30).spacing(20)).into()
        }
        ElementType::Button(name) => Button::new(Text::new(name))
            .on_press(Message::PageButtonPressed)
            .into(),
        ElementType::Hyperlink(location) => Button::new(Text::new(location))
            .on_press(Message::Hyperlink(location.to_string()))
            .into(),
        ElementType::Image(location) => Image::new(location).into(),
    }
}
