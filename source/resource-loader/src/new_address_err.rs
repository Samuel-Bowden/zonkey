use std::fmt;

pub enum NewAddressErr {
    TooManySections,
    InvalidFirstSection,
    Empty,
    InvalidErrType,
}

impl fmt::Display for NewAddressErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::TooManySections => "The provided address contains too many sections. There must be exactly one colon in the address",
            Self::InvalidFirstSection => "The first section must be either 'file', 'zonkey' or 'error'",
            Self::Empty => "The provided address is empty",
            Self::InvalidErrType => "The second section of 'error' was not a valid error type",
        })
    }
}
