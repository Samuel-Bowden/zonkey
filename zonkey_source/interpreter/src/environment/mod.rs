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
    integer_stack_sizes: Vec<usize>,
    float_stack_sizes: Vec<usize>,
    string_stack_sizes: Vec<usize>,
    boolean_stack_sizes: Vec<usize>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            integer_stack: vec![],
            float_stack: vec![],
            string_stack: vec![],
            boolean_stack: vec![],
            integer_stack_sizes: vec![],
            float_stack_sizes: vec![],
            string_stack_sizes: vec![],
            boolean_stack_sizes: vec![],
        }
    }

    pub fn pop_stack(&mut self) {
        interpreter_debug!(format!(
            "Popping int stack at end of block: Before: {:?}",
            self.integer_stack
        )
        .as_str());
        self.integer_stack
            .truncate(self.integer_stack.len() - self.integer_stack_sizes.pop().unwrap());

        interpreter_debug!(format!(
            "Popping float stack at end of block: Before: {:?}",
            self.float_stack
        )
        .as_str());
        self.float_stack
            .truncate(self.float_stack.len() - self.float_stack_sizes.pop().unwrap());

        interpreter_debug!(format!(
            "Popping string stack at end of block: Before: {:?}",
            self.string_stack
        )
        .as_str());
        self.string_stack
            .truncate(self.string_stack.len() - self.string_stack_sizes.pop().unwrap());

        interpreter_debug!(format!(
            "Popping boolean stack at end of block: Before: {:?}",
            self.boolean_stack
        )
        .as_str());
        self.boolean_stack
            .truncate(self.boolean_stack.len() - self.boolean_stack_sizes.pop().unwrap());

        interpreter_debug!(format!("Int stack after: {:?}", self.integer_stack).as_str());
        interpreter_debug!(format!("Float stack after: {:?}", self.float_stack).as_str());
        interpreter_debug!(format!("String stack after: {:?}", self.string_stack).as_str());
        interpreter_debug!(format!("Boolean stack after: {:?}", self.boolean_stack).as_str());
    }

    pub fn push_int(&mut self, integer: i64) {
        self.integer_stack.push(integer);
        *self.integer_stack_sizes.last_mut().unwrap() += 1;
    }

    pub fn push_float(&mut self, float: f64) {
        self.float_stack.push(float);
        *self.float_stack_sizes.last_mut().unwrap() += 1;
    }

    pub fn push_string(&mut self, string: String) {
        self.string_stack.push(string);
        *self.string_stack_sizes.last_mut().unwrap() += 1;
    }

    pub fn push_boolean(&mut self, boolean: bool) {
        self.boolean_stack.push(boolean);
        *self.boolean_stack_sizes.last_mut().unwrap() += 1;
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

        self.integer_stack_sizes.push(0);
        self.float_stack_sizes.push(0);
        self.string_stack_sizes.push(0);
        self.boolean_stack_sizes.push(0);
    }

    pub fn assign_int(
        &mut self,
        id: usize,
        val: i64,
        assignment_operator: &NumericAssignmentOperator,
    ) {
        let current_val = self.integer_stack.get_mut(id).unwrap();

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
        let current_val = self.float_stack.get_mut(id).unwrap();

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
        let current_val = self.string_stack.get_mut(id).unwrap();

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
        let current_val = self.boolean_stack.get_mut(id).unwrap();

        match assignment_operator {
            BooleanAssignmentOperator::Equal => *current_val = val,
        }
    }

    pub fn get_int(&self, id: usize) -> i64 {
        *self.integer_stack.get(id).unwrap()
    }

    pub fn get_float(&self, id: usize) -> f64 {
        *self.float_stack.get(id).unwrap()
    }

    pub fn get_string(&self, id: usize) -> String {
        self.string_stack.get(id).unwrap().clone()
    }

    pub fn get_boolean(&self, id: usize) -> bool {
        *self.boolean_stack.get(id).unwrap()
    }
}
