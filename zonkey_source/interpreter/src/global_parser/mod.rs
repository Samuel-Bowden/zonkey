use rustc_hash::FxHashMap;
use crate::{
    debug_information,
    function::Function,
    function_declaration::FunctionDeclaration,
    interpreter_debug,
    local_parser::{err::ParserErr, LocalParser},
    stmt::Stmt,
    token::Token,
    value_type::ValueType,
};
use std::collections::VecDeque;

pub struct GlobalParser {
    tokens: VecDeque<Token>,
    function_declarations: FxHashMap<String, FunctionDeclaration>,
    unparsed_start: VecDeque<Token>,
    unparsed_functions: VecDeque<(VecDeque<Token>, FunctionDeclaration)>,
    functions: Vec<Function>,
    function_next_id: usize,
    start_already_defined: bool,
}

impl GlobalParser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self {
            tokens,
            function_declarations: FxHashMap::default(),
            unparsed_start: VecDeque::new(),
            unparsed_functions: VecDeque::new(),
            functions: vec![],
            function_next_id: 0,
            start_already_defined: false,
        }
    }

    pub fn run(mut self) -> Result<(Stmt, Vec<Function>), ParserErr> {
        interpreter_debug!("Starting global parser");

        self.scan_global()?;

        self.load_functions()?;

        Ok((self.load_start_block()?, self.functions))
    }

    fn scan_global(&mut self) -> Result<(), ParserErr> {
        loop {
            match self.tokens.front() {
                Some(Token::Function) => self.function_declaration()?,
                Some(Token::Start) => self.start_declaration()?,
                None => break,
                unexpected_token => {
                    panic!("Unexpected token in global scope {:?}", unexpected_token)
                }
            }
        }

        Ok(())
    }

    fn load_functions(&mut self) -> Result<(), ParserErr> {
        while let Some((unparsed_function, function_declaration)) =
            self.unparsed_functions.pop_front()
        {
            let statements = LocalParser::new_function(
                unparsed_function,
                &self.function_declarations,
                &function_declaration,
            )
            .run()?;

            self.functions.push(Function {
                start: Stmt::Block(statements, (0, 0, 0, 0)),
            });
        }

        Ok(())
    }

    fn load_start_block(&mut self) -> Result<Stmt, ParserErr> {
        Ok(Stmt::Block(
            LocalParser::new(
                std::mem::take(&mut self.unparsed_start),
                &self.function_declarations,
            )
            .run()?,
            (0, 0, 0, 0),
        ))
    }

    fn start_declaration(&mut self) -> Result<(), ParserErr> {
        debug_information!("start_declaration");

        if self.start_already_defined {
            panic!("Start already defined");
        }

        self.tokens.pop_front();

        match self.tokens.pop_front() {
            Some(Token::LeftBrace) => (),
            unexpected_token => panic!(
                "Expected '{{' to begin 'start' block, found {:?}",
                unexpected_token
            ),
        }

        let mut other_left_braces = 0;

        loop {
            match self.tokens.pop_front() {
                Some(token) => {
                    match token {
                        Token::LeftBrace => other_left_braces += 1,
                        Token::RightBrace => {
                            if other_left_braces == 0 {
                                break;
                            }
                            other_left_braces -= 1;
                        }
                        _ => (),
                    }
                    self.unparsed_start.push_back(token);
                }
                None => panic!("Expected '}}' to finish 'start' block but the end of the file has been reached"),
            }
        }

        self.start_already_defined = true;

        Ok(())
    }

    fn function_declaration(&mut self) -> Result<(), ParserErr> {
        debug_information!("function_declaration");

        let mut unparsed_tokens = VecDeque::new();

        self.tokens.pop_front();

        let function_name = if let Some(Token::Identifier(identifier)) = self.tokens.pop_front() {
            identifier
        } else {
            return Err(ParserErr::FunctionDeclarationMissingName);
        };

        match self.tokens.pop_front() {
            Some(Token::LeftParen) => (),
            _ => return Err(ParserErr::FunctionDeclarationMissingLeftParen),
        }

        let mut parameters = vec![];

        match self.tokens.front() {
            Some(Token::RightParen) => {
                self.tokens.pop_front();
            }
            _ => loop {
                let parameter_data_type = match self.data_type() {
                    Ok(data_type) => data_type,
                    Err(_) => return Err(ParserErr::FunctionDeclarationParameterBadDataType),
                };

                let parameter_name =
                    if let Some(Token::Identifier(identifier)) = self.tokens.pop_front() {
                        identifier
                    } else {
                        return Err(ParserErr::FunctionDeclarationParameterMissingName);
                    };

                parameters.push((parameter_data_type, parameter_name));

                match self.tokens.pop_front() {
                    Some(Token::Comma) => continue,
                    Some(Token::RightParen) => break,
                    _ => return Err(ParserErr::FunctionDeclarationExpectedCommaOrRightParen),
                }
            },
        }

        let return_data_type = if let Some(Token::Arrow) = self.tokens.front() {
            self.tokens.pop_front();

            match self.tokens.pop_front() {
                Some(Token::IntegerType) => Some(ValueType::Integer),
                Some(Token::FloatType) => Some(ValueType::Float),
                Some(Token::StringType) => Some(ValueType::String),
                Some(Token::BooleanType) => Some(ValueType::Boolean),
                unexpected_token => panic!("Expected parameter type, found {:?}", unexpected_token),
            }
        } else {
            None
        };

        match self.tokens.pop_front() {
            Some(Token::LeftBrace) => (),
            unexpected_token => panic!(
                "Expected '{{' to begin 'function' block, found {:?}",
                unexpected_token
            ),
        }

        let mut other_left_braces = 0;

        loop {
            match self.tokens.pop_front() {
                Some(token) => {
                    match token {
                        Token::LeftBrace => other_left_braces += 1,
                        Token::RightBrace => {
                            if other_left_braces == 0 {
                                break;
                            }
                            other_left_braces -= 1;
                        }
                        _ => (),
                    }
                    unparsed_tokens.push_back(token);
                }
                None => panic!("Expected '}}' to finish 'function' block but the end of the file has been reached"),
            }
        }

        let id = self.function_next_id;

        self.function_next_id += 1;

        self.function_declarations.insert(
            function_name.clone(),
            FunctionDeclaration {
                id,
                parameters: parameters.clone(),
                return_data_type: return_data_type.clone(),
            },
        );

        self.unparsed_functions.push_back((
            unparsed_tokens,
            FunctionDeclaration {
                id,
                parameters,
                return_data_type,
            },
        ));

        Ok(())
    }

    fn data_type(&mut self) -> Result<ValueType, ()> {
        match self.tokens.pop_front() {
            Some(Token::IntegerType) => Ok(ValueType::Integer),
            Some(Token::FloatType) => Ok(ValueType::Float),
            Some(Token::BooleanType) => Ok(ValueType::Boolean),
            Some(Token::StringType) => Ok(ValueType::String),
            _ => Err(()),
        }
    }
}
