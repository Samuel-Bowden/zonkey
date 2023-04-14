use prelude::*;

pub mod array;
mod button;
mod column;
mod hyperlink;
mod image;
mod input;
mod page;
mod prelude;
mod row;
mod text;

pub fn new() -> FxHashMap<Rc<String>, ClassDeclaration> {
    let page = Rc::new("Page".to_string());
    let button = Rc::new("Button".to_string());
    let text = Rc::new("Text".to_string());
    let hyperlink = Rc::new("Hyperlink".to_string());
    let input = Rc::new("Input".to_string());
    let row = Rc::new("Row".to_string());
    let column = Rc::new("Column".to_string());
    let image = Rc::new("Image".to_string());

    let mut classes = FxHashMap::default();

    classes.insert(Rc::clone(&page), page::new(Rc::clone(&page)));
    classes.insert(Rc::clone(&button), button::new(Rc::clone(&button)));
    classes.insert(Rc::clone(&text), text::new(Rc::clone(&text)));
    classes.insert(Rc::clone(&hyperlink), hyperlink::new(Rc::clone(&hyperlink)));
    classes.insert(Rc::clone(&input), input::new(Rc::clone(&input)));
    classes.insert(Rc::clone(&row), row::new(Rc::clone(&row)));
    classes.insert(Rc::clone(&column), column::new(Rc::clone(&column)));
    classes.insert(Rc::clone(&image), image::new(Rc::clone(&image)));

    classes
}
