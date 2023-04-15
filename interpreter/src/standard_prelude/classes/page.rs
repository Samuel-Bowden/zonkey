use super::prelude::*;

pub fn new(page: Rc<String>) -> ClassDeclaration {
    let mut methods = FxHashMap::default();

    methods.insert(
        "add".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Element],
            return_type: Some(ValueType::Class(Rc::clone(&page))),
        }),
    );

    methods.insert(
        "remove".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Element],
            return_type: Some(ValueType::Class(Rc::clone(&page))),
        }),
    );

    methods.insert(
        "center".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Class(Rc::clone(&page))),
        }),
    );

    methods.insert(
        "set_max_width".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float],
            return_type: Some(ValueType::Class(Rc::clone(&page))),
        }),
    );

    methods.insert(
        "set_title".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::clone(&page))),
        }),
    );

    methods.insert(
        "set_background_colour".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::clone(&page))),
        }),
    );

    methods.insert(
        "set_text_colour".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::clone(&page))),
        }),
    );

    ClassDeclaration { methods }
}
