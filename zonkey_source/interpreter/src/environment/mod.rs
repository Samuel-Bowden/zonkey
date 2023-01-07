use crate::{token::Token, tree_walker::value::Value};
use std::collections::HashMap;

pub struct Environment {
    pub stack: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    pub fn get(&self, name: &String) -> Option<&Value> {
        for element in self.stack.iter().rev() {
            if let Some(val) = element.get(name) {
                return Some(val);
            }
        }

        None
    }

    pub fn assign(&mut self, name: &String, value: Value, operator: &Token) {
        for element in self.stack.iter_mut().rev() {
            if let Some(key) = element.get_mut(name) {
                match operator {
                    Token::Equal => *key = value,
                    Token::PlusEqual => *key = (key.clone() + value).unwrap(),
                    Token::MinusEqual => *key = (key.clone() - value).unwrap(),
                    Token::SlashEqual => *key = (key.clone() / value).unwrap(),
                    Token::StarEqual => *key = (key.clone() * value).unwrap(),
                    _ => (),
                }
                break;
            }
        }
    }

    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn insert(&mut self, name: String, value: Value) {
        if let Some(values) = self.stack.last_mut() {
            values.insert(name, value);
        }
    }
}
