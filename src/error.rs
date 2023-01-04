use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum StringValueError {
    ParseError
}

impl Display for StringValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "String cannot be blank")
    }
}

impl std::error::Error for StringValueError {}
