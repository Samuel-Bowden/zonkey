use std::fmt;

pub enum NewAddressErr {
    InvalidFirstSection,
    Empty,
    InvalidErrType,
}

impl fmt::Display for NewAddressErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidFirstSection =>
                    "The first section must be either 'file', 'zonkey', 'http' or 'https'.",
                Self::Empty => "The provided address is empty.",
                Self::InvalidErrType => "The second section of 'error' was not a valid error type.",
            }
        )
    }
}
