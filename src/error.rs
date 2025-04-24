
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CookieParseError(pub String);

impl Display for CookieParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error Parsing Cookie Header, {:?}", self.0)
    }
}
impl Error for CookieParseError {}
