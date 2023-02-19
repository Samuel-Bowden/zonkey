use self::element::{Element, ElementType};

pub mod element;

pub struct ZonkeyApp {
    pub name: String,
    pub root: ElementType,
}

impl ZonkeyApp {
    pub fn new_from_file(name: String) -> Self {
        Self {
            name,
            root: ElementType::Page(vec![]),
        }
    }

    pub fn update(&mut self, event: interpreter::event::Event) {
        if let ElementType::Page(column) = &mut self.root {
            match event {
                interpreter::event::Event::AddHeading(value) => {
                    column.push(Element {
                        id: None,
                        element_type: ElementType::Heading(value),
                    });
                }
                interpreter::event::Event::AddParagraph(value) => {
                    column.push(Element {
                        id: None,
                        element_type: ElementType::Paragraph(value),
                    });
                }
            }
        }
    }
}
