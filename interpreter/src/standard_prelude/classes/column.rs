use super::prelude::*;

pub fn new(column: Rc<String>) -> ClassDeclaration {
    let mut methods = FxHashMap::default();

    methods.insert(
        "add".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Element],
            return_type: Some(ValueType::Class(Rc::clone(&column))),
        }),
    );

    methods.insert(
        "remove".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Element],
            return_type: Some(ValueType::Class(Rc::clone(&column))),
        }),
    );

    methods.insert(
        "set_max_width".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float],
            return_type: Some(ValueType::Class(Rc::clone(&column))),
        }),
    );

    ClassDeclaration { methods }
}
