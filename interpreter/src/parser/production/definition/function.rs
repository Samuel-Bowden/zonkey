use rustc_hash::FxHashMap;

use crate::{
    parser::declaration::CallableDeclaration,
    parser::{declaration::CallableType, production::definition::prelude::*},
};
use std::rc::Rc;

impl Parser {
    pub fn function(&mut self) -> Result<(), ParserStatus> {
        debug_information!("function");

        // First stage - parse function
        let function_token_pos = self.current;
        self.current += 1;

        let function_name = match self.consume_token_type() {
            Some(TokenType::Identifier(name)) => Rc::clone(name),
            _ => {
                self.error
                    .add(ParserErrType::FunctionDeclarationExpectedName(
                        self.tokens[function_token_pos].clone(),
                        self.tokens.get(self.current - 1).cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };

        if let Some(_) = self.function_declarations.get(&function_name) {
            self.error.add(ParserErrType::FunctionRedeclared(
                self.tokens[self.current - 1].clone(),
            ));
            return Err(ParserStatus::Unwind);
        }

        // Get function parameters
        let parameters = self.parameters()?;

        // Get return type if present
        let return_type = self.return_type()?;

        // Second stage - parse function body
        // Add parameters to the first value scope of function body
        let mut function_scope = FxHashMap::default();

        let mut parameter_value_types = vec![];
        for (value_type, name) in parameters {
            self.add_scope_parameter(&value_type, name, &mut function_scope)?;
            parameter_value_types.push(value_type);
        }

        self.environments.push(function_scope);

        let function_declaration = CallableDeclaration {
            callable_type: CallableType::Zonkey(self.callables.len()),
            parameters: parameter_value_types,
            return_type: return_type.clone(),
        };

        self.function_declarations
            .insert(function_name, function_declaration);

        self.current_return_type = return_type;

        // Parse the function block
        let block = self.block()?;

        if let Some(return_type) = &self.current_return_type {
            if !self.returned_value {
                self.error
                    .add(ParserErrType::DeclarationDidNotReturnValueInAllCases(
                        self.tokens[function_token_pos + 1].clone(),
                        return_type.clone(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        }

        // Clean environments after it has been parsed
        self.environments.clear();
        self.integer_next_id = 0;
        self.float_next_id = 0;
        self.string_next_id = 0;
        self.boolean_next_id = 0;
        self.object_next_id = 0;

        self.current_return_type = None;

        // Finally add function to callables
        self.callables.push(block.into());

        Ok(())
    }
}
