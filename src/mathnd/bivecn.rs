use super::{MatN, VecN};
use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
pub struct BiVecN {
    pub ee: Vec<f64>,
}

// Unary minus
impl Neg for BiVecN {
    type Output = BiVecN;
    fn neg(self) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter().map(|x| -x).collect(),
        }
    }
}
impl<'a> Neg for &'a BiVecN {
    type Output = BiVecN;
    fn neg(self) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter().map(|x| -x).collect(),
        }
    }
}

// Vector addition
impl Add for BiVecN {
    type Output = BiVecN;
    fn add(self, v: BiVecN) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter()
                       .zip((v.ee).iter())
                       .map(|(x, y)| x + y)
                       .collect(),
        }
    }
}
impl<'a> Add<BiVecN> for &'a BiVecN {
    type Output = BiVecN;
    fn add(self, v: BiVecN) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter()
                       .zip((v.ee).iter())
                       .map(|(x, y)| x + y)
                       .collect(),
        }
    }
}
impl<'b> Add<&'b BiVecN> for BiVecN {
    type Output = BiVecN;
    fn add(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter()
                       .zip((v.ee).iter())
                       .map(|(x, y)| x + y)
                       .collect(),
        }
    }
}
impl<'a,'b> Add<&'b BiVecN> for &'a BiVecN {
    type Output = BiVecN;
    fn add(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter()
                       .zip((v.ee).iter())
                       .map(|(x, y)| x + y)
                       .collect(),
        }
    }
}

// Vector subtraction
impl Sub for BiVecN {
    type Output = BiVecN;
    fn sub(self, v: BiVecN) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter()
                       .zip((v.ee).iter())
                       .map(|(x, y)| x - y)
                       .collect(),
        }
    }
}
impl<'a> Sub<BiVecN> for &'a BiVecN {
    type Output = BiVecN;
    fn sub(self, v: BiVecN) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter()
                       .zip((v.ee).iter())
                       .map(|(x, y)| x - y)
                       .collect(),
        }
    }
}
impl<'b> Sub<&'b BiVecN> for BiVecN {
    type Output = BiVecN;
    fn sub(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter()
                       .zip((v.ee).iter())
                       .map(|(x, y)| x - y)
                       .collect(),
        }
    }
}
impl<'a,'b> Sub<&'b BiVecN> for &'a BiVecN {
    type Output = BiVecN;
    fn sub(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            ee: (self.ee).iter()
                       .zip((v.ee).iter())
                       .map(|(x, y)| x - y)
                       .collect(),
        }
    }
}

// Scalar multiplication
impl Mul<BiVecN> for f64 {
    type Output = BiVecN;
    fn mul(self, v: BiVecN) -> BiVecN {
        BiVecN {
            ee: v.ee.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl<'b> Mul<&'b BiVecN> for f64 {
    type Output = BiVecN;
    fn mul(self, v: &BiVecN) -> BiVecN {
        BiVecN {
            ee: v.ee.iter()
                .map(|x| self * x)
                .collect(),
        }
    }
}
impl Mul<f64> for BiVecN {
    type Output = BiVecN;
    fn mul(self, s: f64) -> BiVecN {
        BiVecN {
            ee: self.ee.iter()
                .map(|x| x * s)
                .collect(),
        }
    }
}
impl<'a> Mul<f64> for &'a BiVecN {
    type Output = BiVecN;
    fn mul(self, s: f64) -> BiVecN {
        BiVecN {
            ee: self.ee.iter()
                .map(|x| x * s)
                .collect(),
        }
    }
}

// Scalar division
impl Div<f64> for BiVecN {
    type Output = BiVecN;
    fn div(self, s: f64) -> BiVecN {
        BiVecN {
            ee: self.ee.iter()
                .map(|x| x / s)
                .collect(),
        }
    }
}
impl<'a> Div<f64> for &'a BiVecN {
    type Output = BiVecN;
    fn div(self, s: f64) -> BiVecN {
        BiVecN {
            ee: self.ee.iter()
                .map(|x| x / s)
                .collect(),
        }
    }
}

impl BiVecN {
    // Dot product
    pub fn dot(&self, v: &BiVecN) -> f64 {
        (self.ee).iter()
                .zip((v.ee).iter())
                .map(|(&x, &y)| x * y)
                .sum()
    }

    // To MatN

    // Zero
}