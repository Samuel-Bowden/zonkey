use std::sync::{Arc, Mutex};

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
    pub title: String,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
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
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub id: u64,
    pub text: String,
    pub bg_red: f32,
    pub bg_green: f32,
    pub bg_blue: f32,
    pub txt_red: f32,
    pub txt_green: f32,
    pub txt_blue: f32,
    pub clicked: bool,
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
