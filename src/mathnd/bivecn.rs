use super::{MatN, VecN};
use std::ops::{Neg, Add, Sub, Mul, Div};

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
impl Mul<BiVecN> for f64 {
    type Output = BiVecN;
    fn mul(self, v: BiVecN) -> BiVecN {
        BiVecN {
            m: self * v.m,
        }
    }
}
impl<'b> Mul<&'b BiVecN> for f64 {
    type Output = BiVecN;
    fn mul(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: self * &v.m,
        }
    }
}
impl Mul<f64> for BiVecN {
    type Output = BiVecN;
    fn mul(self, s: f64) -> BiVecN {
        BiVecN {
            m: self.m * s,
        }
    }
}
impl<'a> Mul<f64> for &'a BiVecN {
    type Output = BiVecN;
    fn mul(self, s: f64) -> BiVecN {
        BiVecN {
            m: &self.m * s,
        }
    }
}

// Scalar division
impl Div<f64> for BiVecN {
    type Output = BiVecN;
    fn div(self, s: f64) -> BiVecN {
        BiVecN {
            m: self.m / s,
        }
    }
}
impl<'a> Div<f64> for &'a BiVecN {
    type Output = BiVecN;
    fn div(self, s: f64) -> BiVecN {
        BiVecN {
            m: &self.m / s,
        }
    }
}

impl BiVecN {
    // Dot product
    pub fn dot(&self, v: &BiVecN) -> f64 {
        self.m.dot(&v.m) / 2.0
    }

    // Skew
    pub fn skew(&self) -> BiVecN {
        BiVecN {
            m: (&self.m - &self.m.transpose()) / 2.0,
        }
    }

    pub fn get_ij(&self, i: usize, j: usize) -> f64 {
        self.m.e[i].e[j]
    }

    // To MatN
    pub fn to_matn(self) -> MatN {
        self.m
    }

    pub fn to_vecn(&self) -> VecN {
        let mut v: Vec<f64> = Vec::new();
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
            mat.e[i].e[j] = 1.0;
            mat.e[j].e[i] = -1.0;
        }
        Self {
            m: mat,
        }
    }
}