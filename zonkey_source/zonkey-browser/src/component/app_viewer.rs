use crate::{message::Message, zonkey_app::element::ElementType};
use iced::{
    widget::{
        button::{Appearance, StyleSheet},
        Button, Column, Scrollable, Text, TextInput,
    },
    Color, Element, Length,
};
use iced_native::theme;

struct HyperlinkStyle;

impl StyleSheet for HyperlinkStyle {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: None,
            border_width: 0.,
            text_color: Color::from_rgb(0., 0., 1.),
            ..Default::default()
        }
    }
}

struct ButtonStyle {
    bg_red: f32,
    bg_green: f32,
    bg_blue: f32,
}

impl StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(
                self.bg_red,
                self.bg_green,
                self.bg_blue,
            ))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

pub fn build(element_type: &ElementType) -> Element<Message> {
    match element_type {
        ElementType::Text(text) => Text::new(&text.value)
            .size(text.size)
            .style(Color::from([text.red, text.green, text.blue]))
            .into(),
        ElementType::Page(elements) => {
            let mut page_content = vec![];

            for element in elements {
                page_content.push(build(element));
            }

            Scrollable::new(
                Column::with_children(page_content)
                    .padding(30)
                    .spacing(20)
                    .width(Length::Fill),
            )
            .into()
        }
        ElementType::Button(button) => Button::new(Text::new(&button.text))
            .on_press(Message::PageButtonPressed(button.id))
            .style(theme::Button::Custom(Box::new(ButtonStyle {
                bg_red: button.red,
                bg_green: button.green,
                bg_blue: button.blue,
            })))
            .into(),
        ElementType::Hyperlink(text, link) => Button::new(Text::new(text))
            .style(theme::Button::Custom(Box::new(HyperlinkStyle)))
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
