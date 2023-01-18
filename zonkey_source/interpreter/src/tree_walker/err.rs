use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TreeWalkerErr {}

impl Display for TreeWalkerErr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
