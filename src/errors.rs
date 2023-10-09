use std::{num::ParseIntError, fmt::{Formatter, Display}};
pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(msg: String) -> ParseError {
        ParseError { msg }
    }

    pub fn from_str(s: &str) -> ParseError {
        ParseError { msg: s.to_string() }
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError { msg: err.to_string() }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> ParseError {
        ParseError { msg: err.to_string() }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.msg)
    }
}
