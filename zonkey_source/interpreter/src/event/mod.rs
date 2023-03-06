#[derive(Debug, Clone)]
pub enum Event {
    AddHeading(String),
    AddParagraph(String),
    AddButton(String),
    AddHyperlink(String),
    AddImage(String),
}
