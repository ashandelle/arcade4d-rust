use std::{iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub}};

use crate::mathnd::{Abs, FromUsize, MinMaxValue, One, Signum, Sqrt, Two, VecN, Zero};

#[derive(Clone)]
pub struct SimplePolytope<T, const N: usize> {
    // pub center: VecN<T, N>,
    pub zonotopes: Vec<Vec<VecN<T, N>>>,
    pub zextend: Vec<Vec<bool>>,
    pub vertices: Vec<VecN<T, N>>,
    pub vextend: Vec<bool>,
}

impl<T, const N: usize> SimplePolytope<T, N> where T: 
Add<Output = T> +
Mul<Output = T> +
AddAssign +
PartialOrd +
Sum +
Abs + Signum +
Zero + One + MinMaxValue +
FromUsize +
Copy {
    pub fn support(&self, dir: &VecN<T, N>) -> VecN<T, N> {
        let mut dist: T = T::minimum();
        let mut furthest: VecN<T, N> = VecN::zero();

        for i in 0..self.zonotopes.len() {
            let zonotope = &self.zonotopes[i];
            let extentions = &self.zextend[i];
            let mut dot: T = T::zero();
            let mut point: VecN<T, N> = VecN::zero();
            for j in 0..zonotope.len() {
                let generator = &zonotope[j];
                let extend = extentions[j];
                let d: T = generator.dot(*dir);
                if extend {
                    dot += d.abs();
                    point = point + *generator * d.signum();
                } else if d > T::zero() {
                    dot += d;
                    point = point + *generator;
                }
            }
            if dot > dist {
                dist = dot;
                furthest = point;
            }
        }

        for i in 0..self.vertices.len() {
            let vertex = &self.vertices[i];
            let extend = self.vextend[i];
            let dot: T = vertex.dot(*dir);
            if extend && dot.abs() > dist {
                dist = dot.abs();
                furthest = *vertex * dot.signum();
            } else if dot > dist {
                dist = dot;
                furthest = *vertex;
            }
        }

        // furthest + self.center
        furthest
    }

    // pub fn nearest_point(&self, dir: &VecN) -> VecN {

    // }

    // pub fn cube() -> Self {
    //     let mut verts: Vec<VecN<T, N>> = Vec::new();
    //     let mut exts: Vec<bool> = Vec::new();

    //     for i in 0..N {
    //         verts.push(VecN::basis(i));
    //         exts.push(true);
    //     }

    //     Self {
    //         // center: VecN::zero(),
    //         zonotopes: vec![verts],
    //         zextend: vec![exts],
    //         vertices: Vec::new(),
    //         vextend: Vec::new(),
    //     }
    // }

    // pub fn orthoplex() -> Self {
    //     let mut verts: Vec<VecN<T, N>> = Vec::new();
    //     let mut exts: Vec<bool> = Vec::new();

    //     for i in 0..N {
    //         verts.push(VecN::basis(i));
    //         exts.push(true);
    //     }

    //     Self {
    //         // center: VecN::zero(),
    //         zonotopes: Vec::new(),
    //         zextend: Vec::new(),
    //         vertices: verts,
    //         vextend: exts,
    //     }
    // }
}