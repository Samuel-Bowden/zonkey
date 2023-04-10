use super::prelude::*;

pub fn new(row: Rc<String>) -> ClassDeclaration {
    let mut methods = FxHashMap::default();

    methods.insert(
        "add".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Element],
            return_type: Some(ValueType::Class(Rc::clone(&row))),
        }),
    );

    methods.insert(
        "remove".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Element],
            return_type: Some(ValueType::Class(Rc::clone(&row))),
        }),
    );

    methods.insert(
        "center".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Class(Rc::clone(&row))),
        }),
    );

    ClassDeclaration {
        class_type: ClassType::Native,
        methods,
    }
}
