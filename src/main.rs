use crate::mathnd::{BiVecN, MatN, VecN};

mod mathnd;

fn main() {
    let v1: VecN = VecN{e: vec![1.0, 2.0, 3.0, 4.0]};
    let v2: VecN = VecN{e: vec![-1.0, -0.5, 0.0, 3.0]};

    let m1: MatN = MatN::from_vecn(&v1, &v2).orthonormalize();

    let v3 = &m1 * &v1;

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", m1);
    println!("{:?}", v3);
    println!("{:?}", v1.length());
    println!("{:?}", v3.length());
}