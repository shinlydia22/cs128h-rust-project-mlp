// matrix struct
#[derive(Debug, Clone)]
pub struct Matrix {
    num_rows: usize,
    num_cols: usize,
    matrix: Vec<Vec<f64>>,
}

impl<Clone> Matrix {
    // constructor with input height and width
    pub fn new(height: usize, width: usize) -> Self {
        // create 2d height by width vec populated w/ 0.0
        let mut Vec<Vec<f64>> matrix = Vec::with_capacity(height);
        for i in 0..height {
            matrix[i] = vec![0.0; width];
        }
        Matrix {
            num_rows: height,
            num_cols: width,
            matrix: matrix
        }
    }

    // print matrix
    pub fn print_matrix() {
        println!();
    }

    // addition
    pub fn add(other: Matrix) -> Matrix {
        // check if height and width are the same
        if (other.num_cols == num_cols && other.num_rows == num_rows) {

        }
    }
}