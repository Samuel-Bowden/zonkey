use crate::app::{
    element::{Element, ElementType},
    App,
};

pub fn new() -> App {
    App {
        name: String::from("Settings"),
        address: String::from("zonkey:settings"),
        root: ElementType::Page(vec![
            Element {
                id: None,
                element_type: ElementType::Heading("Settings".to_string()),
            },
            Element {
                id: None,
                element_type: ElementType::Paragraph(String::from(
                    "There are no settings to change at the moment...",
                )),
            },
        ]),
    }
}
