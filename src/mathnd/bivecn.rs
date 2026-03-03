use noisy_float::prelude::*;

use super::{MatN, VecN};
use std::{fmt, ops::{Add, Div, Mul, Neg, Sub}};

#[derive(Debug, Clone)]
pub struct BiVecN {
    pub m: MatN,
}

// Unary minus
impl Neg for BiVecN {
    type Output = BiVecN;
    fn neg(self) -> BiVecN {
        BiVecN {
            m: -self.m,
        }
    }
}
impl<'a> Neg for &'a BiVecN {
    type Output = BiVecN;
    fn neg(self) -> BiVecN {
        BiVecN {
            m: -&self.m,
        }
    }
}

// Vector addition
impl Add for BiVecN {
    type Output = BiVecN;
    fn add(self, v: BiVecN) -> BiVecN {
        BiVecN {
            m: self.m + v.m,
        }
    }
}
impl<'a> Add<BiVecN> for &'a BiVecN {
    type Output = BiVecN;
    fn add(self, v: BiVecN) -> BiVecN {
        BiVecN {
            m: &self.m + v.m,
        }
    }
}
impl<'b> Add<&'b BiVecN> for BiVecN {
    type Output = BiVecN;
    fn add(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: self.m + &v.m,
        }
    }
}
impl<'a,'b> Add<&'b BiVecN> for &'a BiVecN {
    type Output = BiVecN;
    fn add(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: &self.m + &v.m,
        }
    }
}

// Vector subtraction
impl Sub for BiVecN {
    type Output = BiVecN;
    fn sub(self, v: BiVecN) -> BiVecN {
        BiVecN {
            m: self.m - v.m,
        }
    }
}
impl<'a> Sub<BiVecN> for &'a BiVecN {
    type Output = BiVecN;
    fn sub(self, v: BiVecN) -> BiVecN {
        BiVecN {
            m: &self.m - v.m,
        }
    }
}
impl<'b> Sub<&'b BiVecN> for BiVecN {
    type Output = BiVecN;
    fn sub(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: self.m - &v.m,
        }
    }
}
impl<'a,'b> Sub<&'b BiVecN> for &'a BiVecN {
    type Output = BiVecN;
    fn sub(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: &self.m - &v.m,
        }
    }
}

// Scalar multiplication
impl Mul<BiVecN> for N64 {
    type Output = BiVecN;
    fn mul(self, v: BiVecN) -> BiVecN {
        BiVecN {
            m: self * v.m,
        }
    }
}
impl<'b> Mul<&'b BiVecN> for N64 {
    type Output = BiVecN;
    fn mul(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: self * &v.m,
        }
    }
}
impl Mul<N64> for BiVecN {
    type Output = BiVecN;
    fn mul(self, s: N64) -> BiVecN {
        BiVecN {
            m: self.m * s,
        }
    }
}
impl<'a> Mul<N64> for &'a BiVecN {
    type Output = BiVecN;
    fn mul(self, s: N64) -> BiVecN {
        BiVecN {
            m: &self.m * s,
        }
    }
}

// Scalar division
impl Div<N64> for BiVecN {
    type Output = BiVecN;
    fn div(self, s: N64) -> BiVecN {
        BiVecN {
            m: self.m / s,
        }
    }
}
impl<'a> Div<N64> for &'a BiVecN {
    type Output = BiVecN;
    fn div(self, s: N64) -> BiVecN {
        BiVecN {
            m: &self.m / s,
        }
    }
}

impl fmt::Display for BiVecN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:?}", self.e)
        let dim = self.m.e.len();
        write!(f, "[");
        for i in 0..dim {
            for j in (i+1)..dim {
                write!(f, "{}", self.m.e[i].e[j]);
                if (i != dim-2) || (j != dim-1) {
                    write!(f, ", ");
                }
            }
        }
        write!(f, "]")
    }
}

impl BiVecN {
    // Dot product
    pub fn dot(&self, v: &BiVecN) -> N64 {
        self.m.dot(&v.m) / 2.0
    }

    // Length
    pub fn length(&self) -> N64 {
        (self.m.length_sqr() / 2.0).sqrt()
    }

    // Length squared
    pub fn length_sqr(&self) -> N64 {
        self.m.length_sqr() / 2.0
    }

    // Skew
    pub fn skew(&self) -> BiVecN {
        BiVecN {
            m: (&self.m - &self.m.transpose()) / n64(2.0),
        }
    }

    pub fn get_ij(&self, i: usize, j: usize) -> N64 {
        self.m.e[i].e[j]
    }

    // To MatN
    pub fn to_matn(self) -> MatN {
        self.m
    }

    pub fn to_vecn(&self) -> VecN {
        let mut v: Vec<N64> = Vec::new();
        let n = self.m.e.len();

        for i in 0..n {
            for j in (i+1)..n {
                v.push(self.get_ij(i, j));
            }
        }

        VecN {
            e: v,
        }
    }

    // Zero
    pub fn zero(dim: usize) -> Self {
        Self {
            m: MatN::zero(dim),
        }
    }

    // Basis element
    pub fn basis(dim: usize, i: usize, j: usize) -> Self {
        let mut mat = MatN::zero(dim);
        if i != j {
            mat.e[i].e[j] = n64(1.0);
            mat.e[j].e[i] = n64(-1.0);
        }
        Self {
            m: mat,
        }
    }
}