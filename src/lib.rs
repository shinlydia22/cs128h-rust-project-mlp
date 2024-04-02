use crate::src::error::{MatrixError, MatrixErrorKind};

// matrix struct
#[derive(Debug, Clone)]
pub struct Matrix {
    num_rows: usize,
    num_cols: usize,
    matrix: Vec<Vec<f64>>,
}

impl Matrix {
    // constructor with input height and width
    pub fn new(height: usize, width: usize) -> Self {
        // create 2d height by width vec populated w/ 0.0
        let mut matrix:Vec<Vec<f64>> = Vec::with_capacity(height);
        for i in 0..height {
            let row = vec![0.0; width];
            matrix.push(row);
        }
        Matrix {
            num_rows: height,
            num_cols: width,
            matrix: matrix
        }
    }

    // print matrix
    pub fn print_matrix(&self) {
        println!("{} x {} matrix:", self.num_rows, self.num_cols);
        for col in 0..self.num_rows {
            for row in 0..self.num_cols {
                print!("{} ", self.matrix[row][col]);
            }
            println!();
        }
    }

    // addition
    pub fn add(&self, other: Matrix) -> Result<Matrix, MatrixError> {
        // check if height and width are the same
        if other.num_cols == self.num_cols && other.num_rows == self.num_rows {
            let error = MatrixError::new(MatrixErrorKind::InvalidDimensions);
            return Err(error);
        }
    }
}