use std::io;
use std::io::Write;

use crate::matrix_error::{MatrixError, MatrixErrorKind};

// asks user for input and returns a tuple of usizes of height and width
pub fn get_dimensions() -> Result<(usize, usize),MatrixError> {
    let mut input = String::new();
    // prompt user and read in string
    print!("input dimensions of the matrix: ([height] [width]) ");
    io::stdout().flush().expect("failed to flush stdout");
    io::stdin().read_line(&mut input).expect("failed to read dimensions");
    // convert to &str and trim
    let binding = String::from(input);
    let mut input_str = binding.trim();
    // split at white space
    let v: Vec<&str> = input_str.split(' ').collect();
    if v.len() != 2 {
        let error = MatrixError::new(MatrixErrorKind::InvalidDimensionsInput);
        return Err(error);
    }
    // convert to usize
    let height: usize = v[0].parse().unwrap();
    let width: usize = v[1].parse().unwrap();
    Ok((height, width))
}