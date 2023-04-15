use crate::{message::Message, ZonkeyBrowser};
use iced::{
    alignment::{Horizontal, Vertical},
    theme,
    widget::{
        button::{Appearance, StyleSheet},
        text, Button, Container, Row, Space,
    },
    Alignment, Background, Length, Theme,
};
use tab::{iced, iced_native::color};

pub enum TabButton {
    Selected,
    NotSelected,
}

impl StyleSheet for TabButton {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> Appearance {
        match self {
            Self::Selected => Appearance {
                background: Some(Background::Color(color!(0xe1e2e2))),
                ..Default::default()
            },
            Self::NotSelected => Appearance {
                background: Some(Background::Color(color!(0xcbcdcd))),
                ..Default::default()
            },
        }
    }
}

pub struct CloseButton;
impl StyleSheet for CloseButton {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: None,
            ..Default::default()
        }
    }
}

pub fn build(browser: &ZonkeyBrowser) -> Container<Message> {
    let mut tab_buttons = vec![];

    for (i, tab) in browser.tabs.iter() {
        tab_buttons.push(
            Button::new(
                Row::new()
                    .push(
                        Container::new(text(tab.title()))
                            .max_height(20.)
                            .width(Length::FillPortion(10)),
                    )
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Button::new(
                            text("x")
                                .size(25)
                                .vertical_alignment(Vertical::Center)
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .width(Length::Fixed(30.))
                        .on_press(Message::TabClosePressed(*i))
                        .style(theme::Button::Custom(Box::new(CloseButton))),
                    )
                    .align_items(Alignment::Center),
            )
            .width(Length::FillPortion(1))
            .on_press(Message::TabPressed(*i))
            .padding(10)
            .style(theme::Button::Custom(Box::new({
                if browser.current_tab == *i {
                    TabButton::Selected
                } else {
                    TabButton::NotSelected
                }
            })))
            .into(),
        );
    }

    let content = Row::new().push(Row::with_children(tab_buttons).width(Length::FillPortion(1)));

    Container::new(content)
        .width(Length::Fill)
        .center_x()
        .max_height(45)
}
