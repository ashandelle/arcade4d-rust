use std::{iter::Sum, ops::{Add, AddAssign, BitXor, Div, Mul, Neg, Sub}};

use crate::mathnd::{Abs, BiVecN, MatN, One, Sqrt, Two, Zero};

#[derive(Debug, Clone, Copy)]
pub struct VecN<T, const N: usize> {
    pub(crate) e: [T; N]
}

impl<T, const N: usize> PartialEq for VecN<T, N> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }
}

impl<T, const N: usize> Neg for VecN<T, N> where T: Neg<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn neg(self) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| -self.e[i]))
    }
}

impl<T, const N: usize> Add for VecN<T, N> where T: Add<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn add(self, v: VecN<T, N>) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i] + v.e[i]))
    }
}

impl<T, const N: usize> Sub for VecN<T, N> where T: Sub<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn sub(self, v: VecN<T, N>) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i] - v.e[i]))
    }
}

// impl<T, const N: usize> Mul<VecN<T, N>> for T where T: Mul<Output = T> + Copy {
//     type Output = VecN<T, N>;
//     fn mul(self, v: VecN<T, N>) -> VecN<T, N> {
//         VecN::new(std::array::from_fn(|i| self * v.e[i]))
//     }
// }
impl<T, const N: usize> Mul<T> for VecN<T, N> where T: Mul<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn mul(self, s: T) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i] * s))
    }
}

impl<T, const N: usize> Div<T> for VecN<T, N> where T: Div<Output = T> + Copy {
    type Output = VecN<T, N>;
    fn div(self, s: T) -> VecN<T, N> {
        VecN::new(std::array::from_fn(|i| self.e[i] / s))
    }
}

impl<T, const N: usize> BitXor for VecN<T, N> where T: Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Two + Copy {
    type Output = BiVecN<T, N>;
    fn bitxor(self, v: VecN<T, N>) -> BiVecN<T, N> {
        // let mut vecs: Vec<VecN> = Vec::new();

        // for i in 0..v.e.len() {
        //     let mut vec: Vec<f64> = Vec::new();
        //     for j in 0..v.e.len() {
        //         vec.push(
        //             self.e[i] * v.e[j] - self.e[j] * v.e[i]
        //         );
        //     }
        //     vecs.push(VecN{e: vec});
        // }

        // BiVecN {
        //     m: MatN{e: vecs},
        // }.skew()
        BiVecN {
            m: MatN::mult_transpose_vecn(self, v) - MatN::mult_transpose_vecn(v, self),
        }.skew()
    }
}

// impl fmt::Display for VecN {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self.e)
//     }
// }

impl<T, const N: usize> VecN<T, N> {
    pub fn new(e: [T; N]) -> Self {
        VecN {
            e: e,
        }
    }

    // Dot product
    pub fn dot(&self, v: VecN<T, N>) -> T where T: Mul<Output = T> + Sum + Copy {
        (self.e).iter()
                .zip((v.e).iter())
                .map(|(&x, &y)| x * y)
                .sum()
    }

    // Left contraction
    pub fn left_contract(&self, v: BiVecN<T, N>) -> VecN<T, N> where T: Neg<Output = T> + Mul<Output = T> + Sum + Copy {
        -(v.m * *self)
    }

    // Normalize
    pub fn normalize(&self) -> VecN<T, N> where T: Mul<Output = T> + Div<Output = T> + Sqrt + Sum + Copy {
        let mag: T = (self.e).iter()
                                .map(|&x| x*x)
                                .sum::<T>().sqrt();
        VecN::new(std::array::from_fn(|i| self.e[i] / mag))
    }

    // Length
    pub fn length(&self) -> T where T: Mul<Output = T> + Sqrt + Sum + Copy {
        (self.e).iter()
                .map(|&x| x*x)
                .sum::<T>().sqrt()
    }

    // Length squared
    pub fn length_sqr(&self) -> T where T: Mul<Output = T> + Sum + Copy {
        (self.e).iter()
                .map(|&x| x*x)
                .sum::<T>()
    }

    pub fn orthonormal_basis(&self) -> [VecN<T, N>; N-1] where T: Neg<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign + PartialOrd + Sum + Sqrt + Abs + Zero + One + Copy {
        let normal: VecN<T, N> = self.normalize();
    
        let mut vecs: [VecN<T, N>; N-1] = [VecN::zero(); N-1];
        let mut maxdot: T = T::zero();
        let mut maxi: usize = 0;
    
        for i in 0..N {
            let d = normal.e[i].abs();
            if d > maxdot {
                maxdot = d;
                maxi = i;
            }
        }
        
        let mut l = 0;
        for i in 0..N {
            if i != maxi {
                let mut v: VecN<T, N> = normal * -normal.e[i];
                v.e[i] += T::one();
                vecs[l] = v;
                l+=1;
            }
        }
    
        for j in 0..(N-1) {
            let vec = vecs[j].normalize();
            for k in j+1..(N-1) {
                vecs[k] = vecs[k] - (vec * vec.dot(vecs[k]));
            }
        }
    
        vecs
    }

    // pub fn to_bivecn(&self) -> BiVecN {
    //     let dim = ((2.0*(self.e.len() as f64) + 0.25).sqrt() + 0.5).round() as usize;
    //     let mut b = BiVecN::zero(dim);
    //
    //     let mut k = 0;
    //     for i in 0..dim {
    //         for j in (i+1)..dim {
    //             b.m.e[i].e[j] = self.e[k];
    //             b.m.e[j].e[i] = -self.e[k];
    //             k+=1;
    //         }
    //     }
    //
    //     b
    // }

    // Zero
    pub fn zero() -> Self where T: Zero + Copy {
        Self::new([T::zero(); N])
    }

    pub fn default() -> Self where T: Default + Copy {
        Self::new([T::default(); N])
    }

    // Basis element
    pub fn basis(element: usize) -> Self where T: Zero + One + Copy {
        let mut vec = Self::zero();
        vec.e[element] = T::one();
        vec
    }
}