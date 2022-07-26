use thiserror::Error;

#[derive(Error, Debug)]
pub enum StringValueError {
    #[error("String cannot be blank")]
    ParseError
}