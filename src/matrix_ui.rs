use std::io;
use std::io::Write;
use mlp::{concat_matrices, Matrix};
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
        self.matrices.insert(name.clone(), mat.clone());
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

    //asks the user for a name for the matrix
    pub fn input_label(&self) -> String {
        print!("What would you like the matrix name to be? ");
        io::stdout().flush().expect("failed to flush stdout");
        let mut label = String::new();
        io::stdin().read_line(&mut label).expect("failed to read name");
        let binding = String::from(label);
        let name = binding.trim();
        return name.to_string();
    }

    //iterates through matrices and prints out the key values,
    //or in other words, the names inputted by the user
    pub fn print_matrices(&self) {
        for label in self.matrices.keys() {
            println!("{}", label);
        }
    }

    //general helper function for retrieving two matrix names from the user
    //returns a matrix error if one of the specified matrices doesn't exist
    pub fn two_inputs(&self) -> Result<(Matrix, Matrix), MatrixError> {
        print!("Please choose with a space in between: [matrix 1] [matrix 2]");
        io::stdout().flush().expect("failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read values");
        let binding = String::from(input);
        let input_str = binding.trim();
        let v: Vec<&str> = input_str.split(' ').collect();
        if v.len() != 2 {
            let error = MatrixError::new(MatrixErrorKind::InvalidInput);
            return Err(error);
        }

        if !self.matrices.contains_key(v[0]) || !self.matrices.contains_key(v[1]) {
            let error = MatrixError::new(MatrixErrorKind::MatrixNotFound);
            return Err(error);
        }

        return Ok((self.matrices.get(v[0]).unwrap().clone(), self.matrices.get(v[1]).unwrap().clone()));
    }

    //general helper function that retrieves a matrix from the user
    //returns a MatrixError if the Matrix they ask for doesn't exist
    pub fn one_input(&self) -> Result<Matrix, MatrixError> {
        print!("Please type in the desired matrix: ");
        io::stdout().flush().expect("failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read value");
        let binding = String::from(input);
        let input_str = binding.trim();
        if !self.matrices.contains_key(input_str) {
            let error = MatrixError::new(MatrixErrorKind::MatrixNotFound);
            return Err(error);
        }

        return Ok(self.matrices.get(input_str).unwrap().clone());
    }

    //Prompts the user if they want to store a created matrix
    pub fn store(&self) -> bool {
        print!("Do you want to store the matrix (y/n)?");
        while true {
            io::stdout().flush().expect("failed to flush stdout");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read values");
            let binding = String::from(input);
            let input_str = binding.trim();
            if input_str == "y" {
                return true;
            } else if input_str == "n" {
                return false;
            }
        }
        return true;
    }

    //Adds a given matrix to the stored list of matrices
    pub fn store_matrix(&mut self, val: Matrix) {
        let label = self.input_label();
        self.matrices.insert(label, val);
    }

    //multiplies two matrices at the request of the user
    pub fn mult_matrices(&mut self) -> Result<Matrix, MatrixError> {
        let mats = self.two_inputs();
        if mats.is_err() {
            let test = mats.unwrap_err();
            return Err(test);
        }
        let v = mats.unwrap();
        let product = v.0.multiply(v.1);
        if product.is_err() {
            let error = product.unwrap_err();
            return Err(MatrixError::new(MatrixErrorKind::InvalidDimensions));
        }
        let mult_mat = product.unwrap();
        let store = self.store();
        if store {
            self.store_matrix(mult_mat.clone());
        }
        return Ok(mult_mat);
    }

    //returns the determinant of a matrix requested by the user
    pub fn determinant(&mut self) -> Result<f64, MatrixError> {
        let mat = self.one_input();
        if mat.is_err() {
            return Err(mat.unwrap_err());
        }
        let det = mat.unwrap().get_determinant();
        if det.is_err() {
            return Err(MatrixError::new(MatrixErrorKind::InvalidDimensions));
        }
        return Ok(det.unwrap());
    }

    //returns the echelon form of a matrix
    pub fn get_echelon(&mut self) -> Result<Matrix, MatrixError> {
        let mat = self.one_input();
        if mat.is_err() {
            return Err(mat.unwrap_err());
        }
        let ech = mat.unwrap().echelon_form();
        let store = self.store();
        if store {self.store_matrix(ech.clone());}
        return Ok(ech);
    }

    //returns the rref of a matrix
    pub fn get_rref(&mut self) -> Result<Matrix, MatrixError> {
        let mat = self.one_input();
        if mat.is_err() {
            return Err(mat.unwrap_err());
        }
        let ech = mat.unwrap().rref();
        let store = self.store();
        if store {self.store_matrix(ech.clone());}
        return Ok(ech);
    }

    //returns the inverse of a matrix unless it is uninvertible
    pub fn inverse(&mut self) -> Result<Matrix, MatrixError> {
        let mat = self.one_input();
        if mat.is_err() {
            return Err(mat.unwrap_err());
        }
        let inv = mat.unwrap().get_inverse();
        if inv.is_err() {
            return Err(MatrixError::new(MatrixErrorKind::Uninvertible));
        }
        let inv_mat = inv.unwrap();
        let store = self.store();
        if store {self.store_matrix(inv_mat.clone());}
        return Ok(inv_mat);
    }

    //returns the concatenation of two matrices
    pub fn get_concat(&mut self) -> Result<Matrix, MatrixError> {
        let mats = self.two_inputs();
        if mats.is_err() {
            let test = mats.unwrap_err();
            return Err(test);
        }
        let v = mats.unwrap();
        let concat = concat_matrices(v.0, v.1);
        if concat.is_err() {
            return Err(MatrixError::new(MatrixErrorKind::InvalidDimensions));
        }
        let concat_mat = concat.unwrap();
        let store = self.store();
        if store {self.store_matrix(concat_mat.clone())}
        return Ok(concat_mat);
    }

    //returns the sum of two matrices
    pub fn get_sum(&mut self) -> Result<Matrix, MatrixError> {
        let mats = self.two_inputs();
        if mats.is_err() {
            let test = mats.unwrap_err();
            return Err(test);
        }
        let v = mats.unwrap();
        let sum = v.0 + v.1;
        if sum.is_err() {
            return Err(MatrixError::new(MatrixErrorKind::InvalidDimensions));
        }
        let sum_mat = sum.unwrap();
        let store = self.store();
        if store {self.store_matrix(sum_mat.clone());}
        return Ok(sum_mat);
    }

    //returns the difference of two matrices
    pub fn get_diff(&mut self) -> Result<Matrix, MatrixError> {
        let mats = self.two_inputs();
        if mats.is_err() {
            let test = mats.unwrap_err();
            return Err(test);
        }
        let v = mats.unwrap();
        let diff = v.0 - v.1;
        if diff.is_err() {
            return Err(MatrixError::new(MatrixErrorKind::InvalidDimensions));
        }
        let diff_mat = diff.unwrap();
        let store = self.store();
        if store {self.store_matrix(diff_mat.clone());}
        return Ok(diff_mat);
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
                G: concatenate two matrices
                H: add two matrices
                I: subtract a matrix from another
                print: print stored matrices");
        Ok(true)
    }

    // takes in String action and does what the action is :)
    //A match block doesn't work since some of these options have different return types
    pub fn do_action(&mut self, action: String) {
        if action == "help" {
            let _ = self.print_options();
        }
        if action == "A" {
            println!("creating matrix");
            let _ = self.input_matrix();
        } else if action == "B" {
            println!("select two matrices to multiply: ");
            self.print_matrices();
            let auto = self.mult_matrices();
            if auto.is_err() {
                println!("{}", auto.unwrap_err().error_msg);
            } else {
                auto.unwrap().print_matrix();
            }
        } else if action == "C" {
            println!("select matrix to get determinant of: ");
            self.print_matrices();
            let auto = self.determinant();
            if auto.is_err() {
                println!("{}", auto.unwrap_err().error_msg);
            } else {
                println!("{}", auto.unwrap());
            }
        } else if action == "D" {
            println!("select matrix to get the echelon form of: ");
            self.print_matrices();
            let auto = self.get_echelon();
            if auto.is_err() {
                println!("{}", auto.unwrap_err().error_msg)
            } else {
                auto.unwrap().print_matrix();
            }
        } else if action == "E" {
            println!("select matrix to get the rref of: ");
            self.print_matrices();
            let auto = self.get_rref();
            if auto.is_err() {
                println!("{}", auto.unwrap_err().error_msg)
            } else {
                auto.unwrap().print_matrix();
            }
        } else if action == "F" {
            println!("select matrix to find the inverse of: ");
            self.print_matrices();
            let auto = self.inverse();
            if auto.is_err() {
                println!("{}", auto.unwrap_err().error_msg)
            } else {
                auto.unwrap().print_matrix();
            }
        } else if action == "G" {
            println!("select two matrices to concatenate: ");
            self.print_matrices();
            let auto = self.get_concat();
            if auto.is_err() {
                println!("{}", auto.unwrap_err().error_msg)
            } else {
                auto.unwrap().print_matrix();
            }
        } else if action == "H" {
            println!("select two matrices to add: ");
            self.print_matrices();
            let auto = self.get_sum();
            if auto.is_err() {
                println!("{}", auto.unwrap_err().error_msg)
            } else {
                auto.unwrap().print_matrix();
            }
        } else if action == "I" {
            println!("select two matrices to subtract: ");
            self.print_matrices();
            let auto = self.get_diff();
            if auto.is_err() {
                println!("{}", auto.unwrap_err().error_msg)
            } else {
                auto.unwrap().print_matrix();
            }
        }
        if action == "print" {
            println!("printing... {} matrices", self.matrices.len());
            self.print_matrices();
        }
    }
}