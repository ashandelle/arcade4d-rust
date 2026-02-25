use super::{BiVecN, VecN};
use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
pub struct MatN {
    pub e: Vec<VecN>,
}

// Unary minus
impl Neg for MatN {
    type Output = MatN;
    fn neg(self) -> MatN {
        MatN {
            e: (self.e).iter().map(|x| -x).collect(),
        }
    }
}
impl<'a> Neg for &'a MatN {
    type Output = MatN;
    fn neg(self) -> MatN {
        MatN {
            e: (self.e).iter().map(|x| -x).collect(),
        }
    }
}

// Matrix addition
impl Add for MatN {
    type Output = MatN;
    fn add(self, v: MatN) -> MatN {
        MatN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| x + y)
                       .collect(),
        }
    }
}
impl<'a> Add<MatN> for &'a MatN {
    type Output = MatN;
    fn add(self, v: MatN) -> MatN {
        MatN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| x + y)
                       .collect(),
        }
    }
}
impl<'b> Add<&'b MatN> for MatN {
    type Output = MatN;
    fn add(self, v: &MatN) -> MatN {
        MatN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| x + y)
                       .collect(),
        }
    }
}
impl<'a,'b> Add<&'b MatN> for &'a MatN {
    type Output = MatN;
    fn add(self, v: &MatN) -> MatN {
        MatN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| x + y)
                       .collect(),
        }
    }
}

// Matrix subtraction
impl Sub for MatN {
    type Output = MatN;
    fn sub(self, v: MatN) -> MatN {
        MatN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| x - y)
                       .collect(),
        }
    }
}
impl<'a> Sub<MatN> for &'a MatN {
    type Output = MatN;
    fn sub(self, v: MatN) -> MatN {
        MatN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| x - y)
                       .collect(),
        }
    }
}
impl<'b> Sub<&'b MatN> for MatN {
    type Output = MatN;
    fn sub(self, v: &MatN) -> MatN {
        MatN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| x - y)
                       .collect(),
        }
    }
}
impl<'a,'b> Sub<&'b MatN> for &'a MatN {
    type Output = MatN;
    fn sub(self, v: &MatN) -> MatN {
        MatN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| x - y)
                       .collect(),
        }
    }
}

// Scalar multiplication
impl Mul<MatN> for f64 {
    type Output = MatN;
    fn mul(self, v: MatN) -> MatN {
        MatN {
            e: v.e.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl<'b> Mul<&'b MatN> for f64 {
    type Output = MatN;
    fn mul(self, v: &MatN) -> MatN {
        MatN {
            e: v.e.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl Mul<f64> for MatN {
    type Output = MatN;
    fn mul(self, s: f64) -> MatN {
        MatN {
            e: self.e.iter()
                .map(|x| x * s)
                .collect(),
        }
    }
}
impl<'a> Mul<f64> for &'a MatN {
    type Output = MatN;
    fn mul(self, s: f64) -> MatN {
        MatN {
            e: self.e.iter()
                .map(|x| x * s)
                .collect(),
        }
    }
}

// Scalar division
impl Div<f64> for MatN {
    type Output = MatN;
    fn div(self, s: f64) -> MatN {
        MatN {
            e: self.e.iter()
                .map(|x| x / s)
                .collect(),
        }
    }
}
impl<'a> Div<f64> for &'a MatN {
    type Output = MatN;
    fn div(self, s: f64) -> MatN {
        MatN {
            e: self.e.iter()
                .map(|x| x / s)
                .collect(),
        }
    }
}

// Vector multiplication
impl Mul<VecN> for MatN {
    type Output = VecN;
    fn mul(self, v: VecN) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| x.dot(&v))
                .collect(),
        }
    }
}
impl<'a> Mul<VecN> for &'a MatN {
    type Output = VecN;
    fn mul(self, v: VecN) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| x.dot(&v))
                .collect(),
        }
    }
}
impl<'b> Mul<&'b VecN> for MatN {
    type Output = VecN;
    fn mul(self, v: &VecN) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| x.dot(&v))
                .collect(),
        }
    }
}
impl<'a,'b> Mul<&'b VecN> for &'a MatN {
    type Output = VecN;
    fn mul(self, v: &VecN) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| x.dot(&v))
                .collect(),
        }
    }
}

// BiVector multiplication
// impl Mul<BiVecN> for MatN {
//     type Output = MatN;
//     // fn mul(self, v: VecN) -> VecN {
//     //     MatN {
//     //         e: self.e.iter()
//     //             .map(|x| x * s)
//     //             .collect(),
//     //     }
//     // }
// }

// Matrix multiplication
impl Mul for MatN {
    type Output = MatN;
    fn mul(self, m: MatN) -> MatN {
        let t: MatN = m.transpose();
        MatN {
            e: self.e.iter()
                .map(|x| &t * x)
                .collect(),
        }
    }
}

impl MatN {
    // To BiVecN
    pub fn to_bivecn(self) -> BiVecN {
        BiVecN {
            m: self,
        }.skew()
    }

    // Orthonormalize

    // Transpose
    pub fn transpose(&self) -> MatN {
        let mut t: Vec<VecN> = Vec::new();
        for i in 0..self.e[0].e.len() {
            let mut v: Vec<f64> = Vec::new();
            for j in 0..self.e.len() {
                v.push(self.e[j].e[i]);
            }
            t.push(VecN{e: v});
        }
        MatN {
            e: t,
        }
    }

    pub fn mult_transpose(&self, v: &VecN) -> VecN {
        VecN {
            e: self.transpose().e.iter()
                .map(|x| x.dot(&v))
                .collect(),
        }
    }

    // Inverse

    // Zero
    pub fn zero(dim: usize) -> Self {
        Self {
            e: vec![VecN::zero(dim); dim],
        }
    }

    // Identity
    pub fn identity(dim: usize) -> Self {
        let mut mat = Self::zero(dim);
        for i in 0..dim {
            mat.e[i].e[i] = 1.0;
        }
        mat
    }
}