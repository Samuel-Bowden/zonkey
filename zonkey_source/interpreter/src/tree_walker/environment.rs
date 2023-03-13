use crate::{
    expr::{
        BooleanAssignmentOperator, NumericAssignmentOperator, ObjectAssignmentOperator,
        StringAssignmentOperator,
    },
    interpreter_debug,
    stack::Stack,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Environment {
    integer_stack: Vec<i64>,
    float_stack: Vec<f64>,
    string_stack: Vec<String>,
    boolean_stack: Vec<bool>,
    object_stack: Vec<Rc<RefCell<Environment>>>,
}

impl Environment {
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
        interpreter_debug!(format!(
            "Popping int stack at end of block: Before: {:#?}",
            self.integer_stack
        )
        .as_str());
        self.integer_stack.truncate(stack.integer);

        interpreter_debug!(format!(
            "Popping float stack at end of block: Before: {:#?}",
            self.float_stack
        )
        .as_str());
        self.float_stack.truncate(stack.float);

        interpreter_debug!(format!(
            "Popping string stack at end of block: Before: {:#?}",
            self.string_stack
        )
        .as_str());
        self.string_stack.truncate(stack.string);

        interpreter_debug!(format!(
            "Popping boolean stack at end of block: Before: {:#?}",
            self.boolean_stack
        )
        .as_str());
        self.boolean_stack.truncate(stack.boolean);

        interpreter_debug!(format!(
            "Popping object stack at end of block: Before: {:#?}",
            self.boolean_stack
        )
        .as_str());
        self.object_stack.truncate(stack.object);

        interpreter_debug!(format!("Int stack after: {:#?}", self.integer_stack).as_str());
        interpreter_debug!(format!("Float stack after: {:#?}", self.float_stack).as_str());
        interpreter_debug!(format!("String stack after: {:#?}", self.string_stack).as_str());
        interpreter_debug!(format!("Boolean stack after: {:#?}", self.boolean_stack).as_str());
        interpreter_debug!(format!("Object stack after: {:#?}", self.object_stack).as_str());
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

    pub fn push_object(&mut self, object: Rc<RefCell<Environment>>) {
        self.object_stack.push(object);
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
        val: Rc<RefCell<Environment>>,
        assignment_operator: &ObjectAssignmentOperator,
    ) {
        let current_val = &mut self.object_stack[id];

        match assignment_operator {
            ObjectAssignmentOperator::Equal => *current_val = val,
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

    pub fn get_object(&self, id: usize) -> Rc<RefCell<Environment>> {
        Rc::clone(&self.object_stack[id])
    }
}
