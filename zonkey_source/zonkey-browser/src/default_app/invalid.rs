use crate::app::{
    element::{Element, ElementType},
    App,
};

pub fn new(address: String) -> App {
    App {
        name: address.clone(),
        address,
        root: ElementType::Page(vec![
            Element {
                id: None,
                element_type: ElementType::Heading("Invalid Address".to_string()),
            },
            Element {
                id: None,
                element_type: ElementType::Paragraph(
                    String::from(
                        "A Zonkey app does not exist at this address."
                    )
                ),
            },
        ]),
    }
}
