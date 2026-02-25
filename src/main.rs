use crate::mathnd::{BiVecN, VecN};

mod mathnd;

fn main() {
    let v1: VecN = VecN{e: vec![1.0, 2.0, 3.0, 4.0]};//.normalize();
    let v2: VecN = VecN{e: vec![-1.0, -0.5, 0.0, 2.0]};//.normalize();
    let v3: VecN = VecN::zero(4);

    let b1: BiVecN = &v1 ^ &v2;

    // -11.25

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", b1);
    println!("{:?}", b1.to_vecn());
    println!("{:?}", v1.left_contract(&b1));
    println!("{:?}", v2.left_contract(&b1));
}