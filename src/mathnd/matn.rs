use noisy_float::prelude::*;

use super::{BiVecN, VecN};
use std::{fmt, ops::{Add, Div, Mul, Neg, Sub}};

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
impl Mul<MatN> for N64 {
    type Output = MatN;
    fn mul(self, v: MatN) -> MatN {
        MatN {
            e: v.e.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl<'b> Mul<&'b MatN> for N64 {
    type Output = MatN;
    fn mul(self, v: &MatN) -> MatN {
        MatN {
            e: v.e.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl Mul<N64> for MatN {
    type Output = MatN;
    fn mul(self, s: N64) -> MatN {
        MatN {
            e: self.e.iter()
                .map(|x| x * s)
                .collect(),
        }
    }
}
impl<'a> Mul<N64> for &'a MatN {
    type Output = MatN;
    fn mul(self, s: N64) -> MatN {
        MatN {
            e: self.e.iter()
                .map(|x| x * s)
                .collect(),
        }
    }
}

// Scalar division
impl Div<N64> for MatN {
    type Output = MatN;
    fn div(self, s: N64) -> MatN {
        MatN {
            e: self.e.iter()
                .map(|x| x / s)
                .collect(),
        }
    }
}
impl<'a> Div<N64> for &'a MatN {
    type Output = MatN;
    fn div(self, s: N64) -> MatN {
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
impl Mul<BiVecN> for MatN {
    type Output = BiVecN;
    fn mul(self, v: BiVecN) -> BiVecN {
        BiVecN {
            m: (&self * &v.m) * &self.transpose(),
        }.skew()
    }
}
impl<'a> Mul<BiVecN> for &'a MatN {
    type Output = BiVecN;
    fn mul(self, v: BiVecN) -> BiVecN {
        BiVecN {
            m: (self * &v.m) * &self.transpose(),
        }.skew()
    }
}
impl<'b> Mul<&'b BiVecN> for MatN {
    type Output = BiVecN;
    fn mul(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: (&self * &v.m) * &self.transpose(),
        }.skew()
    }
}
impl<'a,'b> Mul<&'b BiVecN> for &'a MatN {
    type Output = BiVecN;
    fn mul(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: (self * &v.m) * &self.transpose(),
        }.skew()
    }
}

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

impl fmt::Display for MatN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");
        for i in 0..self.e.len() {
            write!(f, "{}", self.e[i]);
            if i != self.e.len()-1 {
                write!(f, ", ");
            }
        }
        write!(f, "]")
    }
}

impl MatN {
    // Dot product
    pub fn dot(&self, m: &MatN) -> N64 {
        (self.e).iter()
                .zip((m.e).iter())
                .map(|(x, y)| x.dot(&y))
                .sum::<N64>()
    }

    // To BiVecN
    pub fn to_bivecn(self) -> BiVecN {
        BiVecN {
            m: self,
        }.skew()
    }

    // Normalize basis vectors
    pub fn normalize_basis(&self) -> MatN {
        MatN {
            e: (&self.e).iter()
                .map(|x| x.normalize())
                .collect(),
        }
    }

    // Orthonormalize
    pub fn orthonormalize(&self) -> MatN {
        let mut mat = self.normalize_basis();
        let dim = mat.e.len();
        let n = n64((dim - 1).max(2) as f64);

        let mut dot = n64(1.0);

        while dot > n64(1e-12) {
            let mut tmp = mat.clone();

            dot = n64(0.0);
            for i in 0..dim {
                for j in (i+1)..dim {
                    let d = mat.e[i].dot(&mat.e[j]);
                    dot += d.abs();
                    tmp.e[i] = &tmp.e[i] - (&mat.e[j] * d / n);
                    tmp.e[j] = &tmp.e[j] - (&mat.e[i] * d / n);
                }
            }
            dot /= n64(((dim*dim - dim) / 2) as f64);

            mat = tmp.normalize_basis();
        }

        mat
    }

    // Length
    pub fn length(&self) -> N64 {
        (self.e).iter()
                .map(|x| x.length_sqr())
                .sum::<N64>().sqrt()
    }

    // Length squared
    pub fn length_sqr(&self) -> N64 {
        (self.e).iter()
                .map(|x| x.length_sqr())
                .sum::<N64>()
    }

    // Transpose
    pub fn transpose(&self) -> MatN {
        let mut t: Vec<VecN> = Vec::new();
        for i in 0..self.e[0].e.len() {
            let mut v: Vec<N64> = Vec::new();
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

    pub fn mult_transpose_bivecn(&self, v: &BiVecN) -> BiVecN {
        BiVecN {
            m: (self.transpose() * &v.m) * self,
        }.skew()
    }

    // Inverse
    pub fn inverse(&self) -> MatN {
        let dim = self.e.len();
        let mut inv = MatN::identity(dim);

        let mut iter = 0;
        let mut len: N64 = n64(1.0);
        let I = MatN::identity(dim);

        while (len > N64::epsilon()) && (iter < 100) {
            let mut id = &inv * self;
            inv = n64(2.0) * &inv - &id * &inv;
            id = id - &I;
            len = id.dot(&id) / n64((dim*dim) as f64);
            iter+=1;
        }

        inv
    }

    // Matrix rotating v1 to v2
    pub fn from_vecn(v1: &VecN, v2: &VecN) -> Self {
        let dim = v1.e.len();
        let n1 = v1.normalize();
        let n2 = v2.normalize();
        let v3 = &n1 + &n2;
        (MatN::identity(dim) -
        Self::mult_transpose_vecn(&v3, &v3) / (n64(1.0) + &n1.dot(&n2)) +
        n64(2.0) * Self::mult_transpose_vecn(&n2, &n1)) *
        (v2.length() / v1.length())
    }

    // Matrix rotating v1 to v2
    // pub fn from_vecn_interpolate(v1: &VecN, v2: &VecN, t: N64) -> Self {
    //     let dim = v1.e.len();
    //     let v3 = v1 + v2;

    //     let mut cos = v1.dot(&v2).clamp(-1.0, 1.0);
    //     let mut sin = (1.0 - cos*cos).sqrt();
    //     let theta = cos.acos();

    //     if theta.abs() < 1e-8 {
    //         return MatN::identity(dim);
    //     }

    //     let mut c = -Self::mult_transpose_vecn(&v3, &v3) / (1.0 + cos);
    //     let mut s = 2.0 * Self::mult_transpose_vecn(&v2, &v1);

    //     c = c / (1.0 - cos);
    //     s = s / sin;

    //     cos = (theta * t).cos();
    //     sin = (theta * t).sin();

    //     c = c * (1.0 - cos);
    //     s = s * sin;

    //     MatN::identity(dim) + c + s
    // }

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
            mat.e[i].e[i] = n64(1.0);
        }
        mat
    }
}