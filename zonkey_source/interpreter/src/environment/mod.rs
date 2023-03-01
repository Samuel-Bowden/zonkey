use crate::{
    assignment_operator::{
        BooleanAssignmentOperator, NumericAssignmentOperator, StringAssignmentOperator,
    },
    interpreter_debug,
};

pub struct Environment {
    integer_stack: Vec<i64>,
    float_stack: Vec<f64>,
    string_stack: Vec<String>,
    boolean_stack: Vec<bool>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            integer_stack: vec![],
            float_stack: vec![],
            string_stack: vec![],
            boolean_stack: vec![],
        }
    }

    pub fn pop_stack(&mut self, block_start_points: &(usize, usize, usize, usize)) {
        interpreter_debug!(format!(
            "Popping int stack at end of block: Before: {:?}",
            self.integer_stack
        )
        .as_str());
        self.integer_stack.truncate(block_start_points.0);

        interpreter_debug!(format!(
            "Popping float stack at end of block: Before: {:?}",
            self.float_stack
        )
        .as_str());
        self.float_stack.truncate(block_start_points.1);

        interpreter_debug!(format!(
            "Popping string stack at end of block: Before: {:?}",
            self.string_stack
        )
        .as_str());
        self.string_stack.truncate(block_start_points.2);

        interpreter_debug!(format!(
            "Popping boolean stack at end of block: Before: {:?}",
            self.boolean_stack
        )
        .as_str());
        self.boolean_stack.truncate(block_start_points.3);

        interpreter_debug!(format!("Int stack after: {:?}", self.integer_stack).as_str());
        interpreter_debug!(format!("Float stack after: {:?}", self.float_stack).as_str());
        interpreter_debug!(format!("String stack after: {:?}", self.string_stack).as_str());
        interpreter_debug!(format!("Boolean stack after: {:?}", self.boolean_stack).as_str());
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

    pub fn push_stack(&mut self) {
        interpreter_debug!(format!(
            "New int stack at start of block: Current status {:?}",
            self.integer_stack
        )
        .as_str());
        interpreter_debug!(format!(
            "New float stack at start of block: Current status {:?}",
            self.float_stack
        )
        .as_str());
        interpreter_debug!(format!(
            "New string stack at start of block: Current status {:?}",
            self.string_stack
        )
        .as_str());
        interpreter_debug!(format!(
            "New boolean stack at start of block: Current status {:?}",
            self.boolean_stack
        )
        .as_str());
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
}
