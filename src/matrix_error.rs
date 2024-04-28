use std::fmt;

/// Error message for an unrecognized command
const INVALID_DIMENSIONS : &str = "Dimensions do not match.";
const OUT_OF_BOUNDS: &str = "Accessed row/column is out of bounds";
const UNINVERTIBLE: &str = "Matrix can't be inverted";

/// Custom struct for error handling.
#[derive(Debug)]
pub struct MatrixError {
    pub error_type : MatrixErrorKind,
    pub error_msg : String
}

/// Enum to describe the different possible types of errors.
#[derive(Debug, PartialEq)]
pub enum MatrixErrorKind {
    InvalidDimensions,
    OutOfBounds,
    Uninvertible
}

/// MatrixError impl block
impl MatrixError {
    /// Instantiates a new MatrixError object with the given error_type attached
    /// to the given user_input.
    pub fn new(error_type: MatrixErrorKind) -> MatrixError {
        let error_msg = match error_type {
            MatrixErrorKind::InvalidDimensions => INVALID_DIMENSIONS.to_string(),
            MatrixErrorKind::OutOfBounds => OUT_OF_BOUNDS.to_string(),
            MatrixErrorKind::Uninvertible => UNINVERTIBLE.to_string()
        };
        MatrixError {
            error_type,
            error_msg
        }
    }
}

// Implement Display trait for MatrixError
impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MatrixError: {}", self.error_msg)
    }
}

/// Error impl for MatrixError.
impl std::error::Error for MatrixError {}
