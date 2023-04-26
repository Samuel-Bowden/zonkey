use super::prelude::*;

pub fn new(hyperlink: Rc<String>) -> ClassDeclaration {
    let mut methods = FxHashMap::default();

    methods.insert(
        "add_argument".to_string().into(),
        Rc::new(CallableDeclaration {
            callable_type: CallableType::Native,
            parameters: vec![ValueType::String],
            return_type: Some(ValueType::Class(Rc::clone(&hyperlink))),
        }),
    );

    ClassDeclaration { methods }
}
