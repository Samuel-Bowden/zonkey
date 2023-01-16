use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TreeWalkerErr {
    MultipleStartDeclarations,
    InvalidCodeInGlobalScope,
    NoStartDeclaration,
}

impl Display for TreeWalkerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MultipleStartDeclarations => write!(f, "More than one start declaration has been defined"),
            Self::InvalidCodeInGlobalScope => write!(f, "Zonkey source files must not have anything but function and start declarations in the global scope"),
            Self::NoStartDeclaration => write!(f, "No start declaration found"),
        }
    }
}
