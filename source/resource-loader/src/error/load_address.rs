use std::fmt;

#[derive(Debug)]
pub enum LoadAddressErr {
    FileSystemFailure(std::io::Error),
    NetworkFailure(reqwest::Error),
}

impl fmt::Display for LoadAddressErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileSystemFailure(e) => write!(f, "File system failure - {e}"),
            Self::NetworkFailure(e) => write!(f, "Network failure - {e}"),
        }
    }
}
