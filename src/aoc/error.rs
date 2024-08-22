//! Custom error type.

use std::error;
use std::fmt;

/// The custom error type for puzzle input parsing and runner errors.
#[derive(PartialEq)]
pub struct PuzzleError(pub String);

impl error::Error for PuzzleError {}

impl fmt::Debug for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input error: {}", self.0)
    }
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<String> for PuzzleError {
    fn from(msg: String) -> Self {
        Self(msg)
    }
}

impl From<&str> for PuzzleError {
    fn from(msg: &str) -> Self {
        Self(msg.to_owned())
    }
}
