pub mod declaration;
mod production;
mod status;
pub mod value;

use crate::{
    ast::AST,
    err::parser::{ParserErr, ParserErrType},
    parser::declaration::{CallableDeclaration, ClassDeclaration},
    parser::value::Value,
    parser_debug,
    stmt::Stmt,
    token::Token,
};
use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use std::rc::Rc;

use self::{
    declaration::CallableType,
    value::{Object, ValueType},
};

pub struct Parser {
    tokens: Vec<Token>,
    value_stack: Vec<IndexMap<Rc<String>, Value>>,
    integer_next_id: usize,
    float_next_id: usize,
    string_next_id: usize,
    boolean_next_id: usize,
    object_next_id: usize,
    objects: FxHashMap<usize, Rc<Object>>,
    function_declarations: FxHashMap<Rc<String>, Rc<CallableDeclaration>>,
    class_declarations: FxHashMap<Rc<String>, ClassDeclaration>,
    current_return_type: Option<ValueType>,
    callables: Vec<Rc<Stmt>>,
    error: ParserErr,
    start_definition: Option<(Token, Option<Stmt>)>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            value_stack: vec![],
            integer_next_id: 0,
            float_next_id: 0,
            string_next_id: 0,
            boolean_next_id: 0,
            object_next_id: 0,
            objects: FxHashMap::default(),
            function_declarations: FxHashMap::default(),
            class_declarations: FxHashMap::default(),
            current_return_type: None,
            callables: vec![],
            error: ParserErr::new(),
            start_definition: None,
            current: 0,
        }
    }

    pub fn run(mut self) -> Result<AST, ParserErr> {
        parser_debug!("Production rule path:");

        self.add_prelude();

        self.program();

        match (self.start_definition, self.error.had_error()) {
            (Some((_, Some(stmt))), false) => {
                let ast = AST {
                    start: stmt,
                    callable: self.callables,
                };

                parser_debug!("AST");
                #[cfg(debug_assertions)]
                println!("{:#?}", ast);

                Ok(ast)
            }
            (t, _) => {
                if let None = t {
                    self.error.add(ParserErrType::NoStartBlock)
                }
                Err(self.error)
            }
        }
    }

    fn add_prelude(&mut self) {
        // Reference counted strings (to prevent repeated data in memory)
        let button_string = "Button".to_string().into();
        let heading_string = "Heading".to_string().into();
        let paragraph_string = "Paragraph".to_string().into();
        let hyperlink_string = "Hyperlink".to_string().into();
        let input_string = "Input".to_string().into();

        // Functions
        self.function_declarations.insert(
            Rc::new("print".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::Any],
                return_type: None,
            }),
        );

        self.function_declarations.insert(
            Rc::new("println".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::Any],
                return_type: None,
            }),
        );

        self.function_declarations.insert(
            Rc::new("prompt".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::String],
                return_type: Some(ValueType::String),
            }),
        );

        self.function_declarations.insert(
            Rc::new("wait_for_event".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![],
                return_type: Some(ValueType::Boolean),
            }),
        );

        self.function_declarations.insert(
            Rc::new("add_button".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::Class(Rc::clone(&button_string))],
                return_type: None,
            }),
        );

        self.function_declarations.insert(
            Rc::new("add_heading".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::Class(Rc::clone(&heading_string))],
                return_type: None,
            }),
        );

        self.function_declarations.insert(
            Rc::new("add_paragraph".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::Class(Rc::clone(&paragraph_string))],
                return_type: None,
            }),
        );

        self.function_declarations.insert(
            Rc::new("add_hyperlink".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::Class(Rc::clone(&hyperlink_string))],
                return_type: None,
            }),
        );

        self.function_declarations.insert(
            Rc::new("add_input".to_string()),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::Class(Rc::clone(&input_string))],
                return_type: None,
            }),
        );

        // Constructors (a specific type of function)

        // Button constructor
        self.function_declarations.insert(
            Rc::clone(&button_string),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::String],
                return_type: Some(ValueType::Class(Rc::clone(&button_string))),
            }),
        );

        // Heading constructor
        self.function_declarations.insert(
            Rc::clone(&heading_string),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::String],
                return_type: Some(ValueType::Class(Rc::clone(&heading_string))),
            }),
        );

        // Paragraph constructor
        self.function_declarations.insert(
            Rc::clone(&paragraph_string),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::String],
                return_type: Some(ValueType::Class(Rc::clone(&paragraph_string))),
            }),
        );

        // Hyperlink constructor
        self.function_declarations.insert(
            Rc::clone(&hyperlink_string),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::String, ValueType::String],
                return_type: Some(ValueType::Class(Rc::clone(&hyperlink_string))),
            }),
        );

        // Input constructor
        self.function_declarations.insert(
            Rc::clone(&input_string),
            Rc::new(CallableDeclaration {
                callable_type: CallableType::Native,
                parameters: vec![ValueType::String],
                return_type: Some(ValueType::Class(Rc::clone(&hyperlink_string))),
            }),
        );

        // Class declarations

        // Button
        self.class_declarations.insert(Rc::clone(&button_string), {
            let mut properties = FxHashMap::default();

            properties.insert("text".to_string().into(), ValueType::String);
            properties.insert("id".to_string().into(), ValueType::Integer);
            properties.insert("clicked".to_string().into(), ValueType::Boolean);

            let mut methods = FxHashMap::default();

            methods.insert(
                "text".to_string().into(),
                Rc::new(CallableDeclaration {
                    callable_type: CallableType::Native,
                    parameters: vec![
                        ValueType::Class(Rc::clone(&button_string)),
                        ValueType::String,
                    ],
                    return_type: Some(ValueType::Class(Rc::clone(&button_string))),
                }),
            );

            methods.insert(
                "clicked".to_string().into(),
                Rc::new(CallableDeclaration {
                    callable_type: CallableType::Native,
                    parameters: vec![ValueType::Class(Rc::clone(&button_string))],
                    return_type: Some(ValueType::Boolean),
                }),
            );

            ClassDeclaration {
                properties,
                methods,
            }
        });

        // Heading
        self.class_declarations.insert(Rc::clone(&heading_string), {
            let mut properties = FxHashMap::default();

            properties.insert("text".to_string().into(), ValueType::String);
            properties.insert("id".to_string().into(), ValueType::Integer);

            let methods = FxHashMap::default();

            ClassDeclaration {
                properties,
                methods,
            }
        });

        // Paragraph
        self.class_declarations
            .insert(Rc::clone(&paragraph_string), {
                let mut properties = FxHashMap::default();

                properties.insert("text".to_string().into(), ValueType::String);
                properties.insert("id".to_string().into(), ValueType::Integer);

                let methods = FxHashMap::default();

                ClassDeclaration {
                    properties,
                    methods,
                }
            });

        // Hyperlink
        self.class_declarations
            .insert(Rc::clone(&hyperlink_string), {
                let mut properties = FxHashMap::default();

                properties.insert("text".to_string().into(), ValueType::String);
                properties.insert("link".to_string().into(), ValueType::String);
                properties.insert("id".to_string().into(), ValueType::Integer);

                let methods = FxHashMap::default();

                ClassDeclaration {
                    properties,
                    methods,
                }
            });

        // Input
        self.class_declarations.insert(Rc::clone(&input_string), {
            let mut properties = FxHashMap::default();

            properties.insert("text".to_string().into(), ValueType::String);
            properties.insert("id".to_string().into(), ValueType::Integer);

            let mut methods = FxHashMap::default();

            methods.insert(
                "confirmed".to_string().into(),
                Rc::new(CallableDeclaration {
                    callable_type: CallableType::Native,
                    parameters: vec![ValueType::Class(Rc::clone(&input_string))],
                    return_type: None,
                }),
            );

            ClassDeclaration {
                properties,
                methods,
            }
        });
    }
}
