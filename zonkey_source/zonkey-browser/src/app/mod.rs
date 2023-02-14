use self::element::ElementType;

pub mod element;

pub struct App {
    pub name: String,
    pub root: ElementType,
    pub address: String,
}
