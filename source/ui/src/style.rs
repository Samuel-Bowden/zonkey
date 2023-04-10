use crate::element::*;
use iced::{
    widget::{button, container},
    Color,
};

impl container::StyleSheet for Page {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(
                self.red, self.green, self.blue,
            ))),
            ..Default::default()
        }
    }
}

impl button::StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(
                self.bg_red, self.bg_green, self.bg_blue,
            ))),
            text_color: Color::from_rgb(
                self.txt_red, self.txt_green, self.txt_blue,
            ),
            ..Default::default()
        }
    }
}

impl button::StyleSheet for Hyperlink {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            border_width: 0.,
            text_color: Color::from_rgb(0., 0., 1.),
            ..Default::default()
        }
    }
}
