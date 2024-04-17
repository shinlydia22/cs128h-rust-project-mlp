// use matrix_error::{MatrixError, MatrixErrorKind};

mod matrix_error;
use matrix_error::{MatrixError, MatrixErrorKind};

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
        if self.num_cols != other.num_rows {
            let error = MatrixError::new(MatrixErrorKind::InvalidDimensions);
            return Err(error);
        }
        let mut mult_matrix: Matrix = Matrix::new(self.num_rows, other.num_cols);
        for i in 0.. self.num_rows {
            for j in 0.. other.num_cols {
                mult_matrix.matrix[i][j] = dot_product(self.row_vec_at(i).unwrap(), other.col_vec_at(j).unwrap())?;
            }
        }
        return Ok(mult_matrix);
    }

    //determinant
    pub fn get_determinant(&self) -> Result<f64, MatrixError> {
        // check if square
        if self.num_rows != self.num_cols {
            let error = MatrixError::new(MatrixErrorKind::InvalidDimensions);
            return Err(error);
        }

        if self.num_rows == 1 {return Ok(self.matrix[0][0]);}

        let mat = &self.matrix;
        // if 2x2
        if self.num_rows == 2 {
            return Ok((mat[0][0] * mat[1][1]) - (mat[0][1] * mat[1][0]));
        }
        // if bigger than 2x2... cofactor expansion !?!? (across row 1)
        let mut det: f64 = 0.0;
        for i in 0..self.num_rows {
            det += mat[i][1] * self.get_cofactor(i, 1);
        }
        Ok(det)
    }
    
    // cofactor of a matrix (self) given index of row and column (helper for get_determinant)
    fn get_cofactor(&self, row: usize, col: usize) -> f64 {
        // we don't need to do the Result thing bc this not a pub function right
        let minor: Matrix = self.get_minor(row, col);
        let mut cofactor: f64 = 0.0;
        if minor.get_determinant().is_ok() {
            cofactor = minor.get_determinant().unwrap();
        } else {
            // throw some kind of error here !?!?
            // we prob can't even get here tho...
        }
        if (row + col) % 2 == 0 {
            return cofactor;
        } else {
            return cofactor * -1.0;
        }
    }

    // minor (helper for get_cofactor)
    fn get_minor(&self, row: usize, col: usize) -> Matrix {
        // we don't need to do the Result thing bc this not a pub function right
        // make new matrix w dimensions one smaller than self
        let mut minor = Matrix::new(self.num_rows - 1, self.num_cols - 1);
        // iterate thru and copy the stuff over for all places EXCEPT...
        let mut minor_row: usize = 0;
        let mut minor_col: usize = 0;
        for r in 0..self.num_rows {
            if r != row { // for every row except the one we're excluding
                for c in 0..self.num_cols {
                    if c != col { // for every col except the one we're excluding
                        minor.matrix[minor_row][minor_col] = self.matrix[r][c];
                        minor_row += 1;
                        minor_col += 1;
                    }
                }
            }
        }
        minor
    }

    pub fn echelon_form(&self) -> Matrix {
        let mut ech: Matrix = self.clone();
        let diagonals: i32 =  self.num_rows.min(self.num_cols) as i32;



        return ech;
    }

    // returns the element in the matrix at given row and col idx
    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.matrix[row][col]
    }

    // inserts the given f64 into the matrix at given row and col idx
    pub fn insert(&mut self, value: f64, row: usize, col: usize) {
        self.matrix[row][col] = value;
    }

    //For dividing input 1/scaling factor
    pub fn scale_row(&mut self, row_idx: usize, factor: f64) -> Result<(), MatrixError> {
        if row_idx >= self.num_rows {
            let error = MatrixError::new(MatrixErrorKind::OutOfBounds);
            return Err(error);
        }
        for i in 0.. self.num_cols {
            self.matrix[row_idx][i] *= factor;
        }
        return Ok(());
    }

    pub fn row_add(&mut self, row_taken: usize, row_operated: usize, scale: f64) -> Result<(), MatrixError>{
        if row_taken >= self.num_rows || row_operated >= self.num_rows {
            let error = MatrixError::new(MatrixErrorKind::OutOfBounds);
            return Err(error);
        }
        for i in 0.. self.num_cols {
            self.matrix[row_operated][i] += scale * self.matrix[row_taken][i];
        }
        return Ok(());
    }

    pub fn row_sub(&mut self, row_taken: usize, row_operated: usize, scale: f64) -> Result<(), MatrixError>{
        if row_taken >= self.num_rows || row_operated >= self.num_rows {
            let error = MatrixError::new(MatrixErrorKind::OutOfBounds);
            return Err(error);
        }
        for i in 0.. self.num_cols {
            self.matrix[row_operated][i] -= scale * self.matrix[row_taken][i];
        }
        return Ok(());
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

