use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CookieParseError {
    InvalidName,
}

impl Display for CookieParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error Parsing Cookie Header, {:?}",
            match self {
                Self::InvalidName => "Cookie Name contains invalid characters.",
            }
        )
    }
}
impl Error for CookieParseError {}
