// matrix struct
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    num_rows: usize,
    num_cols: usize,
    matrix: Vec<Vec<T>>,
}