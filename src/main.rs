use mlp::Matrix;
// use std::io;
mod matrix_error;
mod util; 
use util::*;

fn main () {

    // more potential names for our future test matrices:
    // - jim
    // - joe
    // - pam
    // - rud
    // - eli
    // - jud
    // - him (like "I'm him" yknow)

    let jud: Matrix = input_matrix().expect("reason ...");
    jud.print_matrix();

    // let mut bob: Matrix = Matrix::new(2,4);
    // bob.insert(5.0,0,0);
    // let vec: Vec<f64> = vec![1.0, 2.0, 4.0, 1.0];
    // let _ = bob.insert_row(1, vec);
    // bob.print_matrix();

    // let mut jim: Matrix = Matrix::new(2,4);
    // jim.insert(5.0, 0, 0);
    // let mut joe: Matrix = Matrix::new(2,4);

    // let pam: Matrix = (joe + jim).expect("REASON");
    // pam.print_matrix();

    // let rud: Matrix = (pam.clone() - bob.clone()).expect("REASON");
    // rud.print_matrix();

    // println!("{}", pam == bob);

    // let mut mat = Matrix::new(3, 3);
    // let row1 = vec![2.0, 4.0, 1.0]; 
    // let row2 = vec![6.0, 15.0, 6.0];
    // let row3 = vec![0.0, 3.0, 7.0];
    // let _ = mat.insert_row(0, row1);
    // let _ = mat.insert_row(1, row2);
    // let _ = mat.insert_row(2, row3);
    // let ech = mat.rref();
    // ech.print_matrix();
}

#[cfg(test)]
mod tests {
    use mlp::Matrix;

    #[test]
    fn test_determinant_2x2() {
        let mut mat: Matrix = Matrix::new(2,2);
        mat.insert(2.0,0,0);
        mat.insert(5.0,1,0);
        mat.insert(3.0,0,1);
        mat.insert(-8.0,1,1);
        // 2 3
        // 5 -8
        let true_det: f64 = -31.0;
        let result = mat.get_determinant();
        assert_eq!(true_det, result.expect("REASON"));
    }

    #[test]
    fn test_minor_4x4() {
        let mut mat: Matrix = Matrix::new(4,4);
        let row1 = vec![1.0, 2.0, 3.0, -2.0];
        let row2 = vec![8.0, -2.0, 3.0, -2.0];
        let row3 = vec![1.0, 0.0, -3.0, 7.0];
        let row4 = vec![0.0, 3.0, 3.0, 6.0];
        mat.insert_row(0, row1);
        mat.insert_row(1, row2);
        mat.insert_row(2, row3);
        mat.insert_row(3, row4);

        let mut true_minor_11: Matrix = Matrix::new(3,3);
        let _ = true_minor_11.insert_row(0, vec![1.0, 3.0, -2.0]);
        let _ = true_minor_11.insert_row(1, vec![1.0, -3.0, 7.0]);
        let _ = true_minor_11.insert_row(2, vec![0.0, 3.0, 6.0]);
        let result_11 = mat.get_minor(1, 1);
        assert_eq!(true_minor_11, result_11);
    }

    #[test]
    fn test_determinant_4x4() {
        let mut mat: Matrix = Matrix::new(4,4);
        let row1 = vec![1.0, 2.0, 3.0, -2.0];
        let row2 = vec![8.0, -2.0, 3.0, -2.0];
        let row3 = vec![1.0, 0.0, -3.0, 7.0];
        let row4 = vec![0.0, 3.0, 3.0, 6.0];
        let _ = mat.insert_row(0, row1);
        let _ = mat.insert_row(1, row2);
        let _ = mat.insert_row(2, row3);
        let _ = mat.insert_row(3, row4);
        let true_det: f64 = 483.0;
        let result = mat.get_determinant();
        assert_eq!(true_det, result.expect("REASON"));
    }

    // #[test]
    // fn test_ref_3x3() {
    //     let mut mat = Matrix::new(3, 3);
    //     let row1 = vec![2.0, 4.0, 1.0];
    //     let row2 = vec![6.0, 15.0, 4,0];
    //     let row3 = vec![0.0, 3.0, 7.0];
    //     let _ = mat.insert_row(0, row1);
    //     let _ = mat.insert_row(1, row2);
    //     let _ = mat.insert_row(2, row3);
    //     let ech = mat.echelon_form();
    //     ech.print_matrix();
    // }
}
