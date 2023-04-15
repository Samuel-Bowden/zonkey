use super::prelude::*;

pub fn new(array_class: Rc<String>, element: ValueType) -> ClassDeclaration {
    let mut methods = FxHashMap::default();

    methods.insert(
        "get".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Integer],
            return_type: Some(element.clone()),
        }),
    );

    methods.insert(
        "push".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![element.clone()],
            return_type: Some(ValueType::Class(array_class.clone())),
        }),
    );

    methods.insert(
        "remove".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Integer],
            return_type: Some(element.clone()),
        }),
    );

    methods.insert(
        "len".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Integer),
        }),
    );

    ClassDeclaration { methods }
}
