use std::{iter::Sum, ops::{Add, Div, Mul, Neg, Sub}};

use crate::mathnd::{MatN, One, Sqrt, Two, Zero};

#[derive(Debug, Clone, Copy)]
pub struct BiVecN<T, const N: usize> {
    pub m: MatN<T, N>,
}

impl<T, const N: usize> PartialEq for BiVecN<T, N> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m
    }
}

// Unary minus
impl<T, const N: usize> Neg for BiVecN<T, N> where T: Neg<Output = T> + Copy {
    type Output = BiVecN<T, N>;
    fn neg(self) -> BiVecN<T, N> {
        BiVecN {
            m: -self.m,
        }
    }
}

// Vector addition
impl<T, const N: usize> Add for BiVecN<T, N> where T: Add<Output = T> + Copy {
    type Output = BiVecN<T, N>;
    fn add(self, v: BiVecN<T, N>) -> BiVecN<T, N> {
        BiVecN {
            m: self.m + v.m,
        }
    }
}

// Vector subtraction
impl<T, const N: usize> Sub for BiVecN<T, N> where T: Sub<Output = T> + Copy {
    type Output = BiVecN<T, N>;
    fn sub(self, v: BiVecN<T, N>) -> BiVecN<T, N> {
        BiVecN {
            m: self.m - v.m,
        }
    }
}

// Scalar multiplication
impl<T, const N: usize> Mul<T> for BiVecN<T, N> where T: Mul<Output = T> + Copy {
    type Output = BiVecN<T, N>;
    fn mul(self, s: T) -> BiVecN<T, N> {
        BiVecN {
            m: self.m * s,
        }
    }
}

// Scalar division
impl<T, const N: usize> Div<T> for BiVecN<T, N> where T: Div<Output = T> + Copy {
    type Output = BiVecN<T, N>;
    fn div(self, s: T) -> BiVecN<T, N> {
        BiVecN {
            m: self.m / s,
        }
    }
}

// impl fmt::Display for BiVecN {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // write!(f, "{:?}", self.e)
//         let dim = self.m.e.len();
//         write!(f, "[");
//         for i in 0..dim {
//             for j in (i+1)..dim {
//                 write!(f, "{}", self.m.e[i].e[j]);
//                 if (i != dim-2) || (j != dim-1) {
//                     write!(f, ", ");
//                 }
//             }
//         }
//         write!(f, "]")
//     }
// }

impl<T, const N: usize> BiVecN<T, N> {
    // Dot product
    pub fn dot(self, b: BiVecN<T, N>) -> T where T: Mul<Output = T> + Div<Output = T> + Sum + Two + Copy {
        self.m.dot(&b.m) / T::two()
    }

    // Length
    pub fn length(&self) -> T where T: Mul<Output = T> + Div<Output = T> + Sqrt + Sum + Two + Copy {
        (self.m.length_sqr() / T::two()).sqrt()
    }

    // Length squared
    pub fn length_sqr(&self) -> T where T: Mul<Output = T> + Div<Output = T> + Sum + Two + Copy {
        self.m.length_sqr() / T::two()
    }

    // Skew
    pub fn skew(&self) -> BiVecN<T, N> where T: Sub<Output = T> + Div<Output = T> + Two + Copy {
        BiVecN {
            m: (self.m - self.m.transpose()) / T::two(),
        }
    }

    pub fn get_ij(&self, i: usize, j: usize) -> T where T: Copy {
        self.m.e[i].e[j]
    }

    // To MatN
    pub fn to_matn(self) -> MatN<T, N> {
        self.m
    }

    // Zero
    pub fn zero() -> Self where T: Zero + Copy {
        Self {
            m: MatN::zero(),
        }
    }

    // Basis element
    pub fn basis(i: usize, j: usize) -> Self where T: Zero + One + Copy {
        let mut mat = MatN::zero();
        if i != j {
            mat.e[i].e[j] = T::one();
            mat.e[j].e[i] = T::one();
        }
        Self {
            m: mat,
        }
    }
}