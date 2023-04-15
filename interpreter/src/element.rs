use std::sync::{Arc, Mutex};
use iced::{
    widget::{button, container},
    Color,
};

#[derive(Debug, Clone)]
pub enum ElementType {
    Text(Arc<Mutex<Text>>),
    Hyperlink(Arc<Mutex<Hyperlink>>),
    Button(Arc<Mutex<Button>>),
    Input(Arc<Mutex<Input>>),
    Row(Arc<Mutex<Row>>),
    Column(Arc<Mutex<Column>>),
    Image(Arc<Mutex<Image>>),
}

#[derive(Debug, Clone)]
pub struct Page {
    pub id: u64,
    pub elements: Vec<(u64, ElementType)>,
    pub bg_red: u8,
    pub bg_green: u8,
    pub bg_blue: u8,
    pub txt_red: u8,
    pub txt_green: u8,
    pub txt_blue: u8,
    pub title: String,
    pub center: bool,
    pub max_width: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub id: u64,
    pub elements: Vec<(u64, ElementType)>,
    pub center: bool,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub id: u64,
    pub elements: Vec<(u64, ElementType)>,
    pub max_width: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub id: u64,
    pub value: String,
    pub size: f32,
    pub colour: Option<(u8, u8, u8)>,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub id: u64,
    pub text: String,
    pub clicked: bool,
    pub bg_red: u8,
    pub bg_green: u8,
    pub bg_blue: u8,
    pub txt_red: u8,
    pub txt_green: u8,
    pub txt_blue: u8,
    pub vertical_padding: f32,
    pub horizontal_padding: f32,
    pub width_fill: bool,
}

#[derive(Debug, Clone)]
pub struct Hyperlink {
    pub id: u64,
    pub text: String,
    pub link: String,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub id: u64,
    pub placeholder: String,
    pub text: String,
    pub confirmed: bool,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub data: Option<Vec<u8>>,
    pub id: u64,
    pub max_width: Option<f32>,
}

impl container::StyleSheet for Page {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(
                self.bg_red, self.bg_green, self.bg_blue,
            ))),
            text_color: Some(Color::from_rgb8(
                self.txt_red, self.txt_green, self.txt_blue,
            )),
            ..Default::default()
        }
    }
}

impl button::StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(
                self.bg_red, self.bg_green, self.bg_blue,
            ))),
            text_color: Color::from_rgb8(
                self.txt_red, self.txt_green, self.txt_blue,
            ),
            ..Default::default()
        }
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(
                self.bg_red.saturating_add(10), self.bg_green.saturating_add(10), self.bg_blue.saturating_add(10),
            ))),
            text_color: Color::from_rgb8(
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
