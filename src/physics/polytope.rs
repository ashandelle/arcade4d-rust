use noisy_float::prelude::*;
use num_traits::Float;

use crate::mathnd::VecN;

#[derive(Clone)]
pub struct Polytope {
    center: VecN,
    zonotopes: Vec<Vec<VecN>>,
    zextend: Vec<Vec<bool>>,
    vertices: Vec<VecN>,
    vextend: Vec<bool>,
}

impl Polytope {
    pub fn support(&self, dir: &VecN) -> VecN {
        let dim = dir.e.len();

        let mut dist: N64 = -N64::infinity(); // Should probably be -inf
        let mut furthest: VecN = VecN::zero(dim);

        for i in 0..self.zonotopes.len() {
            let zonotope = &self.zonotopes[i];
            let extentions = &self.zextend[i];
            let mut dot: N64 = n64(0.0);
            let mut point: VecN = VecN::zero(dim);
            for j in 0..zonotope.len() {
                let generator = &zonotope[j];
                let extend = extentions[j];
                let d: N64 = generator.dot(&dir);
                if extend {
                    dot += d.abs();
                    point = &point + generator * d.signum();
                } else if d > n64(0.0) {
                    dot += d;
                    point = &point + generator;
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
            let dot: N64 = vertex.dot(&dir);
            if extend && dot.abs() > dist {
                dist = dot.abs();
                furthest = vertex.clone() * dot.signum();
            } else if dot > dist {
                dist = dot;
                furthest = vertex.clone();
            }
        }

        furthest + &self.center
    }

    // pub fn nearest_point(&self, dir: &VecN) -> VecN {

    // }

    pub fn cube(dim: usize) -> Self {
        let mut verts: Vec<VecN> = Vec::new();
        let mut exts: Vec<bool> = Vec::new();

        for i in 0..dim {
            verts.push(VecN::basis(dim, i));
            exts.push(true);
        }

        Self {
            center: VecN::zero(dim),
            zonotopes: vec![verts],
            zextend: vec![exts],
            vertices: Vec::new(),
            vextend: Vec::new(),
        }
    }

    pub fn orthoplex(dim: usize) -> Self {
        let mut verts: Vec<VecN> = Vec::new();
        let mut exts: Vec<bool> = Vec::new();

        for i in 0..dim {
            verts.push(VecN::basis(dim, i));
            exts.push(true);
        }

        Self {
            center: VecN::zero(dim),
            zonotopes: Vec::new(),
            zextend: Vec::new(),
            vertices: verts,
            vextend: exts,
        }
    }
}