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
impl<'a> Mul<MatN> for &'a MatN {
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
impl<'b> Mul<&'b MatN> for MatN {
    type Output = MatN;
    fn mul(self, m: &MatN) -> MatN {
        let t: MatN = m.transpose();
        MatN {
            e: self.e.iter()
                .map(|x| &t * x)
                .collect(),
        }
    }
}
impl<'a,'b> Mul<&'b MatN> for &'a MatN {
    type Output = MatN;
    fn mul(self, m: &MatN) -> MatN {
        let t: MatN = m.transpose();
        MatN {
            e: self.e.iter()
                .map(|x| &t * x)
                .collect(),
        }
    }
}

impl MatN {
    // Dot product
    pub fn dot(&self, m: &MatN) -> f64 {
        (self.e).iter()
                .zip((m.e).iter())
                .map(|(x, y)| x.dot(&y))
                .sum::<f64>()
    }

    // To BiVecN
    pub fn to_bivecn(self) -> BiVecN {
        BiVecN {
            m: self,
        }.skew()
    }

    // Orthonormalize
    // pub fn orthonormalize(&self) -> MatN {
        
    // }

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
    pub fn inverse(&self) -> MatN {
        let dim = self.e.len();
        let mut inv = MatN::zero(dim);
        for i in 0..dim {
            inv.e[i].e[i] = (1.0 / self.e[i].e[i]).clamp(-1e8, 1e8);
            if inv.e[i].e[i].is_nan() {
                inv.e[i].e[i] = 0.0;
            }
        }

        let mut iter = 0;
        let mut len: f64 = 1.0;
        let I = MatN::identity(dim);

        while (len > 1e-8) && (iter < 100) {
            let mut id = &inv * self;
            inv = 2.0 * &inv - &id * &inv;

            // let mut nan = false;
            // for i in 0..dim {
            //     for j in 0..dim {
            //         if inv.e[i].e[j].is_nan() {
            //             nan = true;
            //         }
            //     }
            // }
            // if nan {
            //     println!("id: {:?}", id);
            //     println!("inv: {:?}", inv);
            // }

            id = id - &I;
            len = id.dot(&id) / ((dim*dim) as f64);
            // if len < 1e-8 { break; }
            iter+=1;
        }

        inv
    }

    // Matrix rotating v1 to v2
    pub fn from_vecn(v1: &VecN, v2: &VecN) -> Self {
        let dim = v1.e.len();
        let v3 = v1 + v2;
        MatN::identity(dim) -
        Self::mult_transpose_vecn(&v3, &v3) / (1.0 + &v1.dot(&v2)) +
        2.0 * Self::mult_transpose_vecn(&v2, &v1)
    }

    // Matrix rotating v1 to v2
    pub fn from_vecn_interpolate(v1: &VecN, v2: &VecN, t: f64) -> Self {
        let dim = v1.e.len();
        let v3 = v1 + v2;

        let mut cos = v1.dot(&v2).clamp(-1.0, 1.0);
        let mut sin = (1.0 - cos*cos).sqrt();
        let theta = cos.acos();

        if theta.abs() < 1e-8 {
            return MatN::identity(dim);
        }

        let mut c = -Self::mult_transpose_vecn(&v3, &v3) / (1.0 + cos);
        let mut s = 2.0 * Self::mult_transpose_vecn(&v2, &v1);

        c = c / (1.0 - cos);
        s = s / sin;

        cos = (theta * t).cos();
        sin = (theta * t).sin();

        c = c * (1.0 - cos);
        s = s * sin;

        MatN::identity(dim) + c + s
    }

    // Matrix formed by v1 * v2^T
    pub fn mult_transpose_vecn(v1: &VecN, v2: &VecN) -> Self {
        MatN {
            e: (v1.e).iter().map(|x| *x * v2).collect(),
        }
    }

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