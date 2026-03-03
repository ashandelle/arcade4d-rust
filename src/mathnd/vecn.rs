use noisy_float::prelude::*;

use super::{BiVecN, MatN};
use std::{fmt, ops::{Add, BitXor, Div, Mul, Neg, Sub}};

#[derive(Debug, Clone)]
pub struct VecN {
    pub e: Vec<N64>,
}

// Unary minus
impl Neg for VecN {
    type Output = VecN;
    fn neg(self) -> VecN {
        VecN {
            e: (self.e).iter().map(|x| -x).collect(),
        }
    }
}
impl<'a> Neg for &'a VecN {
    type Output = VecN;
    fn neg(self) -> VecN {
        VecN {
            e: (self.e).iter().map(|x| -x).collect(),
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
                       .map(|(x, y)| *x + *y)
                       .collect(),
        }
    }
}
impl<'a> Add<VecN> for &'a VecN {
    type Output = VecN;
    fn add(self, v: VecN) -> VecN {
        VecN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| *x + *y)
                       .collect(),
        }
    }
}
impl<'b> Add<&'b VecN> for VecN {
    type Output = VecN;
    fn add(self, v: &VecN) -> VecN {
        VecN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| *x + *y)
                       .collect(),
        }
    }
}
impl<'a,'b> Add<&'b VecN> for &'a VecN {
    type Output = VecN;
    fn add(self, v: &VecN) -> VecN {
        VecN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| *x + *y)
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
                       .map(|(x, y)| *x - *y)
                       .collect(),
        }
    }
}
impl<'a> Sub<VecN> for &'a VecN {
    type Output = VecN;
    fn sub(self, v: VecN) -> VecN {
        VecN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| *x - *y)
                       .collect(),
        }
    }
}
impl<'b> Sub<&'b VecN> for VecN {
    type Output = VecN;
    fn sub(self, v: &VecN) -> VecN {
        VecN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| *x - *y)
                       .collect(),
        }
    }
}
impl<'a,'b> Sub<&'b VecN> for &'a VecN {
    type Output = VecN;
    fn sub(self, v: &VecN) -> VecN {
        VecN {
            e: (self.e).iter()
                       .zip((v.e).iter())
                       .map(|(x, y)| *x - *y)
                       .collect(),
        }
    }
}

// Scalar multiplication
impl Mul<VecN> for N64 {
    type Output = VecN;
    fn mul(self, v: VecN) -> VecN {
        VecN {
            e: v.e.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl<'b> Mul<&'b VecN> for N64 {
    type Output = VecN;
    fn mul(self, v: &VecN) -> VecN {
        VecN {
            e: v.e.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl Mul<N64> for VecN {
    type Output = VecN;
    fn mul(self, s: N64) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| *x * s)
                .collect(),
        }
    }
}
impl<'a> Mul<N64> for &'a VecN {
    type Output = VecN;
    fn mul(self, s: N64) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| *x * s)
                .collect(),
        }
    }
}

// Scalar division
impl Div<N64> for VecN {
    type Output = VecN;
    fn div(self, s: N64) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| *x / s)
                .collect(),
        }
    }
}
impl<'a> Div<N64> for &'a VecN {
    type Output = VecN;
    fn div(self, s: N64) -> VecN {
        VecN {
            e: self.e.iter()
                .map(|x| *x / s)
                .collect(),
        }
    }
}

// Wedge product
impl BitXor for VecN {
    type Output = BiVecN;
    fn bitxor(self, v: VecN) -> BiVecN {
        let mut vecs: Vec<VecN> = Vec::new();

        for i in 0..v.e.len() {
            let mut vec: Vec<N64> = Vec::new();
            for j in 0..v.e.len() {
                vec.push(
                    self.e[i] * v.e[j] - self.e[j] * v.e[i]
                );
            }
            vecs.push(VecN{e: vec});
        }

        BiVecN {
            m: MatN{e: vecs},
        }.skew()
    }
}
impl<'a> BitXor<VecN> for &'a VecN {
    type Output = BiVecN;
    fn bitxor(self, v: VecN) -> BiVecN {
        let mut vecs: Vec<VecN> = Vec::new();

        for i in 0..v.e.len() {
            let mut vec: Vec<N64> = Vec::new();
            for j in 0..v.e.len() {
                vec.push(
                    self.e[i] * v.e[j] - self.e[j] * v.e[i]
                );
            }
            vecs.push(VecN{e: vec});
        }

        BiVecN {
            m: MatN{e: vecs},
        }.skew()
    }
}
impl<'b> BitXor<&'b VecN> for VecN {
    type Output = BiVecN;
    fn bitxor(self, v: &VecN) -> BiVecN {
        let mut vecs: Vec<VecN> = Vec::new();

        for i in 0..v.e.len() {
            let mut vec: Vec<N64> = Vec::new();
            for j in 0..v.e.len() {
                vec.push(
                    self.e[i] * v.e[j] - self.e[j] * v.e[i]
                );
            }
            vecs.push(VecN{e: vec});
        }

        BiVecN {
            m: MatN{e: vecs},
        }.skew()
    }
}
impl<'a,'b> BitXor<&'b VecN> for &'a VecN {
    type Output = BiVecN;
    fn bitxor(self, v: &VecN) -> BiVecN {
        let mut vecs: Vec<VecN> = Vec::new();

        for i in 0..v.e.len() {
            let mut vec: Vec<N64> = Vec::new();
            for j in 0..v.e.len() {
                vec.push(
                    self.e[i] * v.e[j] - self.e[j] * v.e[i]
                );
            }
            vecs.push(VecN{e: vec});
        }

        BiVecN {
            m: MatN{e: vecs},
        }.skew()
    }
}

impl fmt::Display for VecN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.e)
    }
}

impl VecN {
    // Dot product
    pub fn dot(&self, v: &VecN) -> N64 {
        (self.e).iter()
                .zip((v.e).iter())
                .map(|(&x, &y)| x * y)
                .sum()
    }
    // Left contraction
    pub fn left_contract(&self, v: &BiVecN) -> VecN {
        -(&v.m * self)
    }

    // Normalize
    pub fn normalize(&self) -> VecN {
        let mag: N64 = (self.e).iter()
                                .map(|x| *x*x)
                                .sum::<N64>().sqrt();
        VecN {
            e: self.e.iter()
                    .map(|x| *x / mag)
                    .collect(),
        }
    }

    // Length
    pub fn length(&self) -> N64 {
        (self.e).iter()
                .map(|x| *x*x)
                .sum::<N64>().sqrt()
    }

    // Length squared
    pub fn length_sqr(&self) -> N64 {
        (self.e).iter()
                .map(|x| *x*x)
                .sum::<N64>()
    }

    pub fn orthonormal_basis(&self) -> Vec<VecN> {
        let dim = self.e.len();

        let normal = self.normalize();

        let mut vecs: Vec<VecN> = Vec::new();
        let mut maxdot: N64 = n64(0.0);
        let mut maxi: usize = 0;

        for i in 0..dim {
            let v = VecN::basis(dim, i);
            let d = v.dot(&normal);
            vecs.push(v - &normal * d);
            if d.abs() > maxdot {
                maxdot = d.abs();
                maxi = i;
            }
        }
        vecs.remove(maxi);

        for j in 0..(dim-1) {
            let vec = vecs[j].normalize();
            for k in j+1..(dim-1) {
                vecs[k] = &vecs[k] - (vec.dot(&vecs[k]) * &vec);
            }
        }

        vecs
    }

    pub fn to_bivecn(&self) -> BiVecN {
        let dim = ((2.0*(self.e.len() as f64) + 0.25).sqrt() + 0.5).round() as usize;
        let mut b = BiVecN::zero(dim);

        let mut k = 0;
        for i in 0..dim {
            for j in (i+1)..dim {
                b.m.e[i].e[j] = self.e[k];
                b.m.e[j].e[i] = -self.e[k];
                k+=1;
            }
        }

        b
    }

    // Zero
    pub fn zero(dim: usize) -> Self {
        Self {
            e: vec![n64(0.0); dim],
        }
    }

    // Basis element
    pub fn basis(dim: usize, element: usize) -> Self {
        let mut vec = Self::zero(dim);
        vec.e[element] = n64(1.0);
        vec
    }
}