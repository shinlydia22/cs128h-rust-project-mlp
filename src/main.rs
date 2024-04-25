use mlp::Matrix;

fn main () {
    // more potential names for our future test matrices:
    // - jim
    // - joe
    // - pam
    // - rud
    // - eli
    // - jud
    // - him (like "I'm him" yknow)

    // let mat: Matrix = Matrix::new(1, 1);
    // mat.print_matrix();

    let mut bob: Matrix = Matrix::new(2,4);
    bob.insert(5.0,0,0);
    bob.print_matrix();
    println!("{}", bob.at(0,0));

    let mut jim: Matrix = Matrix::new(2,4);
    jim.insert(5.0, 0, 0);
    let mut joe: Matrix = Matrix::new(2,4);
    let pam: Matrix = (joe + jim).expect("REASON");
    pam.print_matrix();
    let rud: Matrix = (pam.clone() - bob.clone()).expect("REASON");
    rud.print_matrix();

    println!("{}", pam == bob);
}