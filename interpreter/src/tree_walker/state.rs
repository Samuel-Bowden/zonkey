use super::Object;
use crate::{
    expr::{
        BooleanAssignmentOperator, NumericAssignmentOperator, ObjectAssignmentOperator,
        StringAssignmentOperator,
    },
    stack::Stack,
    token::Token,
    tree_walker::err::TreeWalkerErr,
};

#[derive(Debug)]
pub enum NullableReference {
    Some(Object),
    None(Token),
}

#[derive(Debug)]
pub struct State {
    integer_stack: Vec<i64>,
    float_stack: Vec<f64>,
    string_stack: Vec<String>,
    boolean_stack: Vec<bool>,
    object_stack: Vec<NullableReference>,
}

impl State {
    pub fn new() -> Self {
        Self {
            integer_stack: vec![],
            float_stack: vec![],
            string_stack: vec![],
            boolean_stack: vec![],
            object_stack: vec![],
        }
    }

    pub fn pop_stack(&mut self, stack: &Stack) {
        self.integer_stack.truncate(stack.integer);
        self.float_stack.truncate(stack.float);
        self.string_stack.truncate(stack.string);
        self.boolean_stack.truncate(stack.boolean);
        self.object_stack.truncate(stack.object);
    }

    pub fn push_int(&mut self, integer: i64) {
        self.integer_stack.push(integer);
    }

    pub fn push_float(&mut self, float: f64) {
        self.float_stack.push(float);
    }

    pub fn push_string(&mut self, string: String) {
        self.string_stack.push(string);
    }

    pub fn push_boolean(&mut self, boolean: bool) {
        self.boolean_stack.push(boolean);
    }

    pub fn push_object(&mut self, object: NullableReference) {
        self.object_stack.push(object);
    }

    pub fn set_self(&mut self, object: NullableReference) {
        self.object_stack.insert(0, object);
    }

    pub fn assign_int(
        &mut self,
        id: usize,
        val: i64,
        assignment_operator: &NumericAssignmentOperator,
    ) {
        let current_val = &mut self.integer_stack[id];

        match assignment_operator {
            NumericAssignmentOperator::Equal => *current_val = val,
            NumericAssignmentOperator::PlusEqual => *current_val += val,
            NumericAssignmentOperator::MinusEqual => *current_val -= val,
            NumericAssignmentOperator::SlashEqual => *current_val /= val,
            NumericAssignmentOperator::StarEqual => *current_val *= val,
        }
    }

    pub fn assign_float(
        &mut self,
        id: usize,
        val: f64,
        assignment_operator: &NumericAssignmentOperator,
    ) {
        let current_val = &mut self.float_stack[id];

        match assignment_operator {
            NumericAssignmentOperator::Equal => *current_val = val,
            NumericAssignmentOperator::PlusEqual => *current_val += val,
            NumericAssignmentOperator::MinusEqual => *current_val -= val,
            NumericAssignmentOperator::SlashEqual => *current_val /= val,
            NumericAssignmentOperator::StarEqual => *current_val *= val,
        }
    }

    pub fn assign_string(
        &mut self,
        id: usize,
        val: String,
        assignment_operator: &StringAssignmentOperator,
    ) {
        let current_val = &mut self.string_stack[id];

        match assignment_operator {
            StringAssignmentOperator::Equal => *current_val = val,
            StringAssignmentOperator::PlusEqual => *current_val += &val,
        }
    }

    pub fn assign_boolean(
        &mut self,
        id: usize,
        val: bool,
        assignment_operator: &BooleanAssignmentOperator,
    ) {
        let current_val = &mut self.boolean_stack[id];

        match assignment_operator {
            BooleanAssignmentOperator::Equal => *current_val = val,
        }
    }

    pub fn assign_object(
        &mut self,
        id: usize,
        val: Object,
        assignment_operator: &ObjectAssignmentOperator,
    ) {
        let current_val = &mut self.object_stack[id];

        match assignment_operator {
            ObjectAssignmentOperator::Equal => *current_val = NullableReference::Some(val),
        }
    }

    pub fn get_int(&self, id: usize) -> i64 {
        self.integer_stack[id]
    }

    pub fn get_float(&self, id: usize) -> f64 {
        self.float_stack[id]
    }

    pub fn get_string(&self, id: usize) -> String {
        self.string_stack[id].clone()
    }

    pub fn get_boolean(&self, id: usize) -> bool {
        self.boolean_stack[id]
    }

    pub fn get_object(&self, id: usize) -> Result<Object, TreeWalkerErr> {
        match &self.object_stack[id] {
            NullableReference::Some(object) => Ok(object.clone()),
            NullableReference::None(prop_name) => {
                Err(TreeWalkerErr::PropertyNotInitialised(prop_name.clone()))
            }
        }
    }
}
