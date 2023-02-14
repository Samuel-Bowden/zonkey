use crate::app::{
    element::{Element, ElementType},
    App,
};

pub fn new() -> App {
    App {
        name: String::from("Home"),
        address: String::from("zonkey:home"),
        root: ElementType::Page(vec![
            Element {
                id: None,
                element_type: ElementType::Heading("Home".to_string()),
            },
            Element {
                id: None,
                element_type: ElementType::Paragraph(
                    String::from(
                        "Welcome to the Zonkey Browser. Open your first app with the text box on the address bar."
                    )
                ),
            },
        ]),
    }
}
