use crate::parser::declaration::ClassDeclaration;
use rustc_hash::FxHashMap;
use std::rc::Rc;

mod button;
mod hyperlink;
mod input;
mod page;
mod text;

pub fn new() -> FxHashMap<Rc<String>, ClassDeclaration> {
    let page = Rc::new("Page".to_string());
    let button = Rc::new("Button".to_string());
    let text = Rc::new("Text".to_string());
    let hyperlink = Rc::new("Hyperlink".to_string());
    let input = Rc::new("Input".to_string());

    let mut classes = FxHashMap::default();

    classes.insert(Rc::clone(&page), page::new(Rc::clone(&page)));
    classes.insert(Rc::clone(&button), button::new(Rc::clone(&button)));
    classes.insert(Rc::clone(&text), text::new(Rc::clone(&text)));
    classes.insert(Rc::clone(&hyperlink), hyperlink::new(Rc::clone(&hyperlink)));
    classes.insert(Rc::clone(&input), input::new(Rc::clone(&input)));

    classes
}
