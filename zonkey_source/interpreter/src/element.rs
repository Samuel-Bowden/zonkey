use iced::{
    widget::{button, container},
    Color,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum ElementType {
    Text(Arc<Mutex<Text>>),
    Hyperlink(Arc<Mutex<Hyperlink>>),
    Button(Arc<Mutex<Button>>),
    Input(Arc<Mutex<Input>>),
    Row(Arc<Mutex<Row>>),
    Column(Arc<Mutex<Column>>),
}

#[derive(Debug, Clone)]
pub struct Page {
    pub elements: Vec<ElementType>,
    pub title: String,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

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

#[derive(Debug, Clone)]
pub struct Row {
    pub elements: Vec<ElementType>,
    pub center: bool,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub elements: Vec<ElementType>,
    pub max_width: f32,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub value: String,
    pub size: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub clicked: bool,
    pub vertical_padding: f32,
    pub horizontal_padding: f32,
    pub width_fill: bool,
}

impl button::StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(
                self.red, self.green, self.blue,
            ))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hyperlink {
    pub text: String,
    pub link: String,
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

#[derive(Debug, Clone)]
pub struct Input {
    pub placeholder: String,
    pub text: String,
    pub confirmed: bool,
}
