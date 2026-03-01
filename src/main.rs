use crate::mathnd::{BiVecN, MatN, VecN};

use rand::Rng;
use rand_distr::StandardNormal;

mod mathnd;
mod physics;

fn main() {
    let mut rng = rand::rng();

    let v1: VecN = VecN{e: vec![rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal)]};
    let v2: VecN = VecN{e: vec![rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal)]};
    let v3: VecN = VecN{e: vec![rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal)]};
    let v4: VecN = VecN{e: vec![rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal)]};

    let m1: MatN = MatN::from_vecn(&v1, &v2);

    let b1: BiVecN = &v3 ^ &v4;
    let b2: BiVecN = (&m1 * &v3) ^ (&m1 * &v4);

    println!("{:?}", m1 * b1);
    println!("{:?}", b2);
}