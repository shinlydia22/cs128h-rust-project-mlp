use std::io;
use std::io::Write;
use mlp::Matrix;
use std::collections::HashMap;

use crate::matrix_error::{MatrixError, MatrixErrorKind};

// MatrixUI struct that contains a map mapping a name to a matrix
pub struct MatrixUI {
    matrices: HashMap<String, Matrix>,
}

impl MatrixUI {
    // constructor
    pub fn new() -> Self {
        MatrixUI {
            matrices: HashMap::new()
        }
    }

    // asks user for input and returns a tuple of usizes of height and width
    fn input_dimensions(&self) -> Result<(usize, usize),MatrixError> {
        let mut input = String::new();
        // prompt user and read in string
        print!("input dimensions of the matrix: ([height] [width]) ");
        io::stdout().flush().expect("failed to flush stdout");
        io::stdin().read_line(&mut input).expect("failed to read dimensions");
        // convert to &str and trim
        let binding = String::from(input);
        let input_str = binding.trim();
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
    fn fill_matrix(&mut self, num_rows: usize, num_cols: usize) -> Result<Matrix, MatrixError> {
        // initialize empty matrix
        let mut mat: Matrix = Matrix::new(num_rows, num_cols);
        println!("Now we are going to fill in the matrix row by row!");
        println!("input each row as the values separated by a space :)");
        for row in 0..num_rows {
            println!("current row index: {}", row);
            let row_in = self.input_row(num_cols).expect("hahah");
            let _ = mat.insert_row(row, row_in);
        }
        Ok(mat)
    }

    // prompts user to input a row given num_cols (length of row)
    fn input_row(&mut self, num_cols: usize) -> Result<Vec<f64>, MatrixError> {
        print!("input row ({} values): ", num_cols);
        io::stdout().flush().expect("failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read row values");
        // convert to &str and trim
        let binding = String::from(input);
        let input_str = binding.trim();
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
    pub fn input_matrix(&mut self) -> Result<Matrix, MatrixError> {
        //get name of matrix
        let name = self.input_label();
        // get dimensions of the matrix
        let dimensions = self.input_dimensions().unwrap();
        let num_rows = dimensions.0;
        let num_cols = dimensions.1;

        // create the matrix and fill it in
        let mat: Matrix = self.fill_matrix(num_rows, num_cols).expect("REASON");
        self.matrices.insert(name, mat.clone());
        Ok(mat)
    }

    // ask the user what they want to do next and returns their input
    pub fn input_action(&self) -> String {
        // prompt user
        print!("what would you like to do next? (enter \"help\" for options or \"quit\") ");
        io::stdout().flush().expect("failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read row values");
        // convert to &str and trim
        let binding = String::from(input);
        let input_str = binding.trim();
        input_str.to_string()
        
    }

    pub fn input_label(&self) -> String {
        print!("What would you like the matrix name to be?");
        io::stdout().flush().expect("failed to flush stdout");
        let mut label = String::new();
        io::stdin().read_line(&mut label).expect("failed to read name");
        let binding = String::from(label);
        let name = binding.trim();
        return name.to_string();
    }

    // prints options for the user to know what they can do with their matrices
    fn print_options(&self) -> Result<bool, MatrixError> {
        println!("
                A: create new matrix
                B: multiply two matrices
                C: get determinant of matrix
                D: get echelon form of matrix
                E: get rref of matrix
                F: get inverse of matrix
                G: find dot product of two matrices
                H: concatenate two matrices
                I: add two matrices
                J: subtract a matrix from another");
        Ok(true)
    }

    // takes in String action and does what the action is :)
    pub fn do_action(&mut self, action: String) {
        if action == "help" {
            let _ = self.print_options();
        }
        if action == "A" {
            println!("creating matrix");
            let _ = self.input_matrix();
        } 
    }
}