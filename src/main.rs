use crate::mathnd::{BiVecN, MatN, VecN};

mod mathnd;

fn main() {
    let v1: VecN = VecN{e: vec![1.0, 2.0, 3.0, 4.0]}.normalize();
    let v2: VecN = VecN{e: vec![-1.0, -0.5, 0.0, 2.0]}.normalize();

    let m1: MatN = MatN::from_vecn_interpolate(&v1, &v2, 0.75);

    let v3 = &m1 * &v1;

    let t1 = v1.dot(&v3);
    let t2 = v2.dot(&v3);

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t1.acos() + t2.acos());
}