use std::error::Error;
use std::fmt;

//
// Error types --------------------------
//

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.msg)
    }
}

impl Error for ParseError {}

impl ParseError {
    pub fn new<T: AsRef<str>>(msg: T) -> ParseError {
        ParseError {
            msg: msg.as_ref().to_string(),
        }
    }
}

//
// Helper Functions ---------------------
//
