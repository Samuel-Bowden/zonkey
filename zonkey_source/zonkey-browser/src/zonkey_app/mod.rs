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
                InterpreterEvent::AddText(text) => {
                    elements.push(ElementType::Text(text));
                }
                InterpreterEvent::AddButton(button) => {
                    elements.push(ElementType::Button(button));
                }
                InterpreterEvent::AddHyperlink(text, link, _) => {
                    elements.push(ElementType::Hyperlink(text, link));
                }
                InterpreterEvent::SetButtonText(new_text, id) => {
                    if let ElementType::Button(button) = &mut elements[id as usize] {
                        button.text = new_text
                    }
                }
                InterpreterEvent::SetTextValue(new_value, id) => {
                    if let ElementType::Text(current_text) = &mut elements[id as usize] {
                        current_text.value = new_value;
                    }
                }
                InterpreterEvent::SetTextSize(new_size, id) => {
                    if let ElementType::Text(current_text) = &mut elements[id as usize] {
                        current_text.size = new_size;
                    }
                }
                InterpreterEvent::SetTextColour(red, green, blue, id) => {
                    if let ElementType::Text(current_text) = &mut elements[id as usize] {
                        current_text.red = red;
                        current_text.green = green;
                        current_text.blue = blue;
                    }
                }
                InterpreterEvent::AddInput(text, id) => {
                    elements.push(ElementType::Input(id, text, "".to_string()))
                }
                InterpreterEvent::SetButtonBackgroundColour(red, green, blue, id) => {
                    if let ElementType::Text(current_button) = &mut elements[id as usize] {
                        current_button.red = red;
                        current_button.green = green;
                        current_button.blue = blue;
                    }
                }
            }
        }
    }
}
