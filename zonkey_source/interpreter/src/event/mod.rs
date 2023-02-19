#[derive(Debug, Clone)]
pub enum Event {
    AddHeading(String),
    AddParagraph(String),
}
