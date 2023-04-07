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

        // Get function parameters
        let parameters = self.parameters(function_token_pos)?;

        // Get return type if present
        let return_type = self.return_type()?;

        // Second stage - parse function body
        // Add parameters to the first value scope of function body
        let mut function_scope = IndexMap::new();

        let mut parameter_value_types = vec![];
        for (value_type, name) in parameters {
            self.add_scope_parameter(&value_type, name, &mut function_scope)?;
            parameter_value_types.push(value_type);
        }

        self.value_stack.push(function_scope);

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

        // Clean value stack after it has been parsed
        self.value_stack.clear();
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
