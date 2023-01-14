use crate::{
    function::{Function, NativeFunction},
    tree_walker::{
        err::TreeWalkerErr,
        status::TreeWalkerStatus,
        value::{Value, ValueType},
    },
};
use std::collections::HashMap;

pub fn insert(function_stack: &mut HashMap<String, Box<dyn Function>>) {
    function_stack.insert(
        "printLine".to_string(),
        Box::new(NativeFunction {
            function: Box::new(print_line),
            parameters: vec![ValueType::String],
            name: "printLine".to_string(),
        }),
    );

    function_stack.insert(
        "print".to_string(),
        Box::new(NativeFunction {
            function: Box::new(print),
            parameters: vec![ValueType::String],
            name: "printLine".to_string(),
        }),
    );

    function_stack.insert(
        "printInteger".to_string(),
        Box::new(NativeFunction {
            function: Box::new(print_integer),
            parameters: vec![ValueType::Integer],
            name: "printInteger".to_string(),
        }),
    );

    function_stack.insert(
        "printLineInteger".to_string(),
        Box::new(NativeFunction {
            function: Box::new(print_line_integer),
            parameters: vec![ValueType::Integer],
            name: "printLineInteger".to_string(),
        }),
    );

    function_stack.insert(
        "newLine".to_string(),
        Box::new(NativeFunction {
            function: Box::new(new_line),
            parameters: vec![],
            name: "newLine".to_string(),
        }),
    );
}

fn print_line(arguments: &Vec<Value>) -> Result<TreeWalkerStatus, TreeWalkerErr> {
    println!("{}", arguments[0]);

    Ok(TreeWalkerStatus::Ok)
}

fn print(arguments: &Vec<Value>) -> Result<TreeWalkerStatus, TreeWalkerErr> {
    print!("{}", arguments[0]);

    Ok(TreeWalkerStatus::Ok)
}

fn print_integer(arguments: &Vec<Value>) -> Result<TreeWalkerStatus, TreeWalkerErr> {
    print!("{}", arguments[0]);

    Ok(TreeWalkerStatus::Ok)
}

fn print_line_integer(arguments: &Vec<Value>) -> Result<TreeWalkerStatus, TreeWalkerErr> {
    println!("{}", arguments[0]);

    Ok(TreeWalkerStatus::Ok)
}

fn new_line(_arguments: &Vec<Value>) -> Result<TreeWalkerStatus, TreeWalkerErr> {
    println!();

    Ok(TreeWalkerStatus::Ok)
}
