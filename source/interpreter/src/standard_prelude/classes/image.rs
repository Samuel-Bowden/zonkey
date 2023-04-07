use super::prelude::*;

pub fn new(image: Rc<String>) -> ClassDeclaration {
    let mut methods = FxHashMap::default();

    methods.insert(
        "set_max_width".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::Float],
            return_type: Some(ValueType::Class(Rc::clone(&image))),
        }),
    );

    ClassDeclaration {
        class_type: ClassType::Native,
        methods,
    }
}
