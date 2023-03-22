use super::prelude::*;

pub fn new(_: Rc<String>) -> ClassDeclaration {
    let mut methods = FxHashMap::default();

    methods.insert(
        "confirmed".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::Boolean),
        }),
    );

    methods.insert(
        "get_text".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![],
            return_type: Some(ValueType::String),
        }),
    );

    ClassDeclaration {
        class_type: ClassType::Native,
        methods,
    }
}
