pub struct Element {
    pub id: Option<String>,
    pub element_type: ElementType,
}

pub enum ElementType {
    Heading(String),
    Paragraph(String),
    Hyperlink(String),
    Image(String),
    Page(Vec<Element>),
    Button(String),
}
