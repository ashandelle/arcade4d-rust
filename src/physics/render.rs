use std::{iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub}};

use crate::physics::Body;

use mathnd::{vecn::VecN, traits::{MinMax, Sqrt, Two}};
use num_traits::{Bounded, FromPrimitive, Signed, Zero};

#[derive(Clone)]
pub enum Render<T, const N: usize> {
    HalfSpace { normal: VecN<T, N> },
    Sphere { radius: T },
    Box { dimensions: VecN<T, N> },
    Orthoplex { radius: T },
}

impl<T, const N: usize> Render<T, N> where T: 
Neg<Output = T> + Add<Output = T> + Sub<Output = T> +
Mul<Output = T> + Div<Output = T> +
AddAssign + DivAssign +
PartialOrd + MinMax +
Sum +
Sqrt + Signed +
Zero + Two + Bounded +
FromPrimitive +
Copy {
    pub fn sdf(&self, body: &Body<T, N>, vec: &VecN<T, N>) -> T {
        match self {
            Self::HalfSpace { normal } => {
                (*vec - body.pos.linear).dot(*normal)
            },
            Self::Sphere { radius } => {
                (*vec - body.pos.linear).length() - *radius
            },
            Self::Box { dimensions } => {
                let mut p = body.world_pos_to_body(vec);
                // p.e = (p.e).iter()
                //     .zip((dimensions.e).iter())
                //     .map(|(x,y)| x.abs() - y)
                //     .collect();
                for (i, elem) in p.e.iter_mut().enumerate() {
                    *elem = elem.abs() - dimensions.e[i];
                }
                // let m2 = match (p.e).iter().max() {
                //     Some(value) => *value,
                //     None => T::zero(),
                // }.min(T::zero());
                let mut m = T::min_value();
                for (i, elem) in p.e.iter().enumerate() {
                    m = m.max(*elem);
                }
                m = m.min(T::zero());
                // p.e = (p.e).iter().map(|x| x.max(T::zero())).collect();
                for (i, elem) in p.e.iter_mut().enumerate() {
                    *elem = elem.max(T::zero());
                }
                p.length() + m
            },
            Self::Orthoplex { radius } => {
                let p = body.world_pos_to_body(vec);
                let sum = (p.e).iter()
                    .map(|x| x.abs())
                    .sum::<T>();
                (sum - *radius) / T::from_usize(N).unwrap().sqrt()
            },
        }
    }
}