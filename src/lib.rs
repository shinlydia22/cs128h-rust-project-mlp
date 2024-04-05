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
        if other.num_cols != self.num_cols && other.num_rows != self.num_rows {
            let error = MatrixError::new(MatrixErrorKind::InvalidDimensions);
            return Err(error);
        }
        let mut sum_matrix: Matrix = Matrix::new(self.num_rows, self.num_cols);
        for row in 0.. self.num_rows {
            for col in 0.. self.num_cols {
                sum_matrix.matrix[row][col] = self.matrix[row][col] + other.matrix[row][col];
            }
        }
        return Ok(sum_matrix);
    }

    //subtraction
    pub fn subtract(&self, other: Matrix) -> Result<Matrix, MatrixError> {
        if other.num_cols != self.num_cols && other.num_rows != self.num_rows {
            let error = MatrixError::new(MatrixErrorKind::InvalidDimensions);
            return Err(error);
        }
        let mut sub_matrix: Matrix = Matrix::new(self.num_rows, self.num_cols);
        for row in 0.. self.num_rows {
            for col in 0.. self.num_cols {
                sub_matrix.matrix[row][col] = self.matrix[row][col] - other.matrix[row][col];
            }
        }
        return Ok(sub_matrix);
    }

    pub fn row_vec_at(&self, row: usize) -> Result<Vec<f64>, MatrixError>{
        if row >= self.num_rows {
            //Look at this later once we add another error type
        }
        let mut row_vec: Vec<f64> = Vec::new();
        for i in 0.. self.num_cols {
            row_vec.push(self.matrix[row][i]);
        }
        return Ok(row_vec);
    }

    pub fn col_vec_at(&self, col: usize) -> Result<Vec<f64>, MatrixError> {
        if col >= self.num_cols {
            //Add new error here later
        }
        let mut col_vec: Vec<f64> = Vec::new();
        for i in 0.. self.num_rows {
            col_vec.push(self.matrix[i][col]);
        }
        return Ok(col_vec);
    }

    //multiplication
    pub fn multiply(&self, other: Matrix) -> Result<Matrix, MatrixError> {
        if(self.num_cols != other.num_rows) {
            let error = MatrixError::new(MatrixErrorKind::InvalidDimensions);
            return Err(error);
        }
        let mut mult_matrix: Matrix = Matrix::new(self.num_rows, other.num_cols);
        for i in 0.. self.num_rows {
            for j in 0.. other.num_cols {
                mult_matrix.matrix[i][j] = dot_product(self.row_vec_at(i), other.col_vec_at(j));
            }
        }
        return Ok(mult_matrix);
    }

    //determinant
    pub fn get_determinant(&self) -> Result<f64, MatrixError> {
        if self.num_rows != self.num_cols {
            let error = MatrixError::new(MatrixErrorKind::InvalidDimensions);
            return Err(error);
        }
        if self.num_rows == 1 {return self.matrix[0][0];}
    }


}

pub fn dot_product(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<f64, MatrixError> {
    if vec1.len() != vec2.len() {
        let error = MatrixError::new(MatrixErrorKind::InvalidDimensions);
        return Err(error);
    }
    let mut dot_product: f64 = 0.0;
    for i in 0.. vec1.len() {
        dot_product += vec1[i] * vec2[1];
    }
    return Ok(dot_product);
 }