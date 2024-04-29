use std::io;
use std::io::Write;
use mlp::Matrix;

use crate::matrix_error::{MatrixError, MatrixErrorKind};

// asks user for input and returns a tuple of usizes of height and width
fn input_dimensions() -> Result<(usize, usize),MatrixError> {
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
        let error = MatrixError::new(MatrixErrorKind::InvalidInput);
        return Err(error);
    }
    // convert to usize
    let height: usize = v[0].parse().unwrap();
    let width: usize = v[1].parse().unwrap();
    Ok((height, width))
}

// takes in dimensiosn and then prompts the user to fill in the matrix one row at a time
fn fill_matrix(num_rows: usize, num_cols: usize) -> Result<Matrix, MatrixError> {
    // initialize empty matrix
    let mut mat: Matrix = Matrix::new(num_rows, num_cols);
    println!("Now we are going to fill in the matrix row by row!");
    println!("input each row as the values separated by a space :)");
    for row in 0..num_rows {
        println!("current row index: {}", row);
        let row_in = input_row(num_cols).expect("hahah");
        let _ = mat.insert_row(row, row_in);
    }
    Ok(mat)
}

// prompts user to input a row given num_cols (length of row)
fn input_row(num_cols: usize) -> Result<Vec<f64>, MatrixError> {
    print!("input row ({} values): ", num_cols);
    io::stdout().flush().expect("failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read row values");
    // convert to &str and trim
    let binding = String::from(input);
    let mut input_str = binding.trim();
    // split at white space
    let v: Vec<&str> = input_str.split(' ').collect();
    if v.len() != num_cols {
        let error = MatrixError::new(MatrixErrorKind::InvalidInput);
        return Err(error);
    }
    // convert to f64
    let mut v_f64: Vec<f64> = Vec::new();
    for i in 0..v.len() {
        let val: f64 = v[i].parse().unwrap();
        v_f64.push(val);
    }
    Ok(v_f64)
}

// create a matrix using user input
pub fn input_matrix() -> Result<Matrix, MatrixError> {
    // get dimensions of the matrix
    let dimensions = input_dimensions().unwrap();
    let num_rows = dimensions.0;
    let num_cols = dimensions.1;
    // println!("ur dimensions are {} and {}", num_rows, num_cols);

    // create the matrix and fill it in
    let mut mat: Matrix = fill_matrix(num_rows, num_cols).expect("REASON");
    // jud.print_matrix();
    Ok(mat)
}

// ask the user what they want to do next and proceed accordingly
pub fn input_action() {
    // prompt user
    print!("what would you like to do next? (enter \"help\" for options) ");
    io::stdout().flush().expect("failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read row values");
    // convert to &str and trim
    let binding = String::from(input);
    let input_str = binding.trim();
    match input_str {
        "help" => print_options(),
        _ => println!("input does not match any options"),
    }
    
}

// prints options for the user to know what they can do with their matrices
fn print_options() {
    println!("A: create new matrix
              B: multiply two matrices
              C: get determinant of matrix
              D: get echelon form of matrix
              E: get rref of matrix
              F: get inverse of matrix
              G: find dot product of two matrices
              H: concatenate two matrices
              I: add two matrices
              J: subtract a matrix from another");
}