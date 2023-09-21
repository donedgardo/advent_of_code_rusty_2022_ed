use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Failed to parse.")]
    Fail,
    #[error("Failed to parse empty.")]
    Empty,
}
