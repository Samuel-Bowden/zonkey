use super::prelude::*;

pub fn new(text: Rc<String>) -> ClassDeclaration {
    let mut methods = FxHashMap::default();

    methods.insert(
        "set_text".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::clone(&text))),
        }),
    );

    methods.insert(
        "set_size".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float],
            return_type: Some(ValueType::Class(Rc::clone(&text))),
        }),
    );

    methods.insert(
        "set_colour".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::clone(&text))),
        }),
    );

    ClassDeclaration { methods }
}
