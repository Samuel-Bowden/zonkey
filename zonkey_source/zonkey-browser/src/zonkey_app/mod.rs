use interpreter::event::InterpreterEvent;

use self::element::ElementType;

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

    pub fn update(&mut self, event: InterpreterEvent) {
        if let ElementType::Page(elements) = &mut self.root {
            match event {
                InterpreterEvent::AddHeading(text, _) => {
                    elements.push(ElementType::Heading(text));
                }
                InterpreterEvent::AddParagraph(text, _) => {
                    elements.push(ElementType::Paragraph(text));
                }
                InterpreterEvent::AddButton(text, id) => {
                    elements.push(ElementType::Button(id, text));
                }
                InterpreterEvent::AddHyperlink(text, link, _) => {
                    elements.push(ElementType::Hyperlink(text, link));
                }
                InterpreterEvent::ChangeButtonText(new_text, id) => {
                    if let ElementType::Button(_, button_text) = &mut elements[id as usize] {
                        *button_text = new_text;
                    }
                }
                InterpreterEvent::AddInput(text, id) => {
                    elements.push(ElementType::Input(id, text, "".to_string()))
                }
            }
        }
    }
}
