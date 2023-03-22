use super::prelude::*;

pub fn new(_: Rc<String>) -> ClassDeclaration {
    let methods = FxHashMap::default();

    ClassDeclaration {
        class_type: ClassType::Native,
        methods,
    }
}
