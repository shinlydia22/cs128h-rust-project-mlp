use mlp::Matrix;

fn main () {
    // more potential names for our future test matrices:
    // - jim
    // - joe
    // - pam
    // - rud
    // - him (like "I'm him" yknow)

    let mat: Matrix = Matrix::new(1, 1);
    mat.print_matrix();

    let mut bob: Matrix = Matrix::new(2,2);
    bob.insert(5.0,0,0);
    bob.print_matrix();
    print!("{}", bob.at(0,0));
}