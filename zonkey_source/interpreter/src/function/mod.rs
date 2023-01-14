use crate::{
    environment::Environment,
    global::Global,
    stmt::Stmt,
    tree_walker::{
        err::TreeWalkerErr,
        status::TreeWalkerStatus,
        value::{Value, ValueType},
        TreeWalker,
    },
};

pub trait Function {
    fn call(
        &self,
        arguments: &Vec<Value>,
        global: &Global,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr>;
}

pub struct NativeFunction {
    pub function: Box<dyn Fn(&Vec<Value>) -> Result<TreeWalkerStatus, TreeWalkerErr>>,
    pub parameters: Vec<ValueType>,
    pub name: String,
}

impl Function for NativeFunction {
    fn call(
        &self,
        arguments: &Vec<Value>,
        _global: &Global,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        if arguments.len() != self.parameters.len() {
            return Err(TreeWalkerErr::CallIncorrectArity(
                self.name.clone(),
                self.parameters.len(),
                arguments.len(),
            ));
        }

        let mut arguments_iter = arguments.iter();
        let mut parameters_iter = self.parameters.iter();
        let mut count = 0;

        loop {
            if count == arguments.len() {
                break;
            }

            let argument = arguments_iter.next().unwrap();
            let parameter = parameters_iter.next().unwrap();

            if argument.get_value_type() != *parameter {
                return Err(TreeWalkerErr::CallArgumentIncompatibleTypes(
                    self.name.clone(),
                    count,
                ));
            }

            count += 1;
        }

        (self.function)(arguments)
    }
}

pub struct ZonkeyFunction {
    pub block: Box<Stmt>,
    pub parameters: Vec<(ValueType, String)>,
    pub name: String,
}

impl Function for ZonkeyFunction {
    fn call(
        &self,
        arguments: &Vec<Value>,
        global: &Global,
    ) -> Result<TreeWalkerStatus, TreeWalkerErr> {
        if arguments.len() != self.parameters.len() {
            return Err(TreeWalkerErr::CallIncorrectArity(
                self.name.clone(),
                self.parameters.len(),
                arguments.len(),
            ));
        }

        let mut environment = Environment::new();
        let mut arguments_iter = arguments.iter();
        let mut parameters_iter = self.parameters.iter();
        let mut count = 0;

        loop {
            if count == arguments.len() {
                break;
            }

            let argument = arguments_iter.next().unwrap();
            let parameter = parameters_iter.next().unwrap();

            if argument.get_value_type() != parameter.0 {
                return Err(TreeWalkerErr::CallArgumentIncompatibleTypes(
                    self.name.clone(),
                    count,
                ));
            }

            environment.insert(parameter.1.clone(), argument.clone());

            count += 1;
        }

        TreeWalker::new(&mut environment, global).interpret(&self.block)
    }
}
