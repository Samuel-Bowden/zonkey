pub enum ElementType {
    Heading(String),
    Paragraph(String),
    Hyperlink(String, String),
    Page(Vec<ElementType>),
    Button(i64, String),
    Input(i64, String, String),
}
