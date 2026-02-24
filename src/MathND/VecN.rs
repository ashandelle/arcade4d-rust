use super::{BiVecN, MatN};
use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct VecN {
    pub e: Vec<f64>,
}

// Unary minus
impl Neg for VecN {
    type Output = VecN;
    fn neg(self) -> VecN {
        VecN {
            e: (self.e).iter().map(|&x| -x).collect(),
        }
    }
}

// Vector addition
impl Add for VecN {
    type Output = VecN;
    fn add(self, v: VecN) -> VecN {
        VecN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(&x, &y)| x + y)
                       .collect(),
        }
    }
}

// Vector subtraction
impl Sub for VecN {
    type Output = VecN;
    fn sub(self, v: VecN) -> VecN {
        VecN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(&x, &y)| x - y)
                       .collect(),
        }
    }
}

// Scalar multiplication
impl Mul<VecN> for f64 {
    type Output = VecN;
    fn mul(self, v: VecN) -> VecN {
        VecN {
            e: v.e.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl Mul<f64> for VecN {
    type Output = VecN;
    fn mul(self, s: f64) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| x * s)
                .collect(),
        }
    }
}

// Scalar division
impl Div<f64> for VecN {
    type Output = VecN;
    fn div(self, s: f64) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| x / s)
                .collect(),
        }
    }
}

impl VecN {
    // Dot product
    pub fn dot(&self, v: &VecN) -> f64 {
        (self.e).iter()
                .zip((v.e).iter())
                .map(|(&x, &y)| x * y)
                .sum()
    }

    // Normalize
    pub fn normalize(&self) -> self {
        let mag: f64 = (self.e).iter()
                                .map(|&x| x*x)
                                .sum();
        mag = mag.sqrt();
        Self {
            e: self.e.iter()
                .map(|x| x / mag)
                .collect(),
        }
    }

    // Wedge product

    // Zero
    // pub fn zero(dim: usize) -> Self {
    //     Self {
    //         e: vec![0.0; dim],
    //     }
    // }
    pub fn zero() -> Self {
        Self {
            e: self.e.fill(0.0),
        }
    }
}