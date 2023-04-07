use std::fmt;

#[derive(Debug)]
pub enum LoadAddressErr {
    FailedToLoadFile(std::io::Error),
}

impl fmt::Display for LoadAddressErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FailedToLoadFile(e) => write!(f, "{e}"),
        }
    }
}
