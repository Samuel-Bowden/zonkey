use crate::parser::production::prelude::*;
use indexmap::IndexMap;

impl Parser {
    pub fn start(&mut self) -> Result<(), ParserStatus> {
        debug_information!("start");

        let start_token = self.tokens[self.current].clone();
        self.current += 1;

        // Add start value scope
        self.value_stack.push(IndexMap::new());

        let block = self.block();

        // Clean value stack after it has been parsed
        self.value_stack.clear();
        self.integer_next_id = 0;
        self.float_next_id = 0;
        self.string_next_id = 0;
        self.boolean_next_id = 0;

        if let Some((t, _)) = &self.start_definition {
            self.error
                .add(ParserErrType::RedefinedStart(t.clone(), start_token));
            return Err(ParserStatus::Unwind);
        }

        let (start, result) = match block {
            Ok(block) => (Some((start_token, Some(block))), Ok(())),
            Err(e) => (Some((start_token, None)), Err(e)),
        };

        self.start_definition = start;
        result
    }
}
