use interpreter::event::{Button, Text};

pub enum ElementType {
    Text(Text),
    Hyperlink(String, String),
    Page(Vec<ElementType>),
    Button(Button),
    Input(i64, String, String),
}
