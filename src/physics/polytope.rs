use noisy_float::prelude::*;
use num_traits::Float;

use crate::mathnd::VecN;

struct Polytope {
    center: VecN,
    zonotopes: Vec<Vec<VecN>>,
    zextend: Vec<Vec<bool>>,
    vertices: Vec<VecN>,
    vextend: Vec<bool>,
}

impl Polytope {
    fn support(&self, dir: &VecN) -> VecN {
        let dim = dir.e.len();

        let mut dist: N64 = -N64::infinity(); // Should probably be -inf
        let mut furthest: VecN = VecN::zero(dim);

        for zonotope in &self.zonotopes {
            let mut dot: N64 = n64(0.0);
            let mut point: VecN = VecN::zero(dim);
            for generator in zonotope {
                let d: N64 = generator.dot(&dir);
                if d > n64(0.0) {
                    dot += d;
                    point = &point + generator;
                }
            }
            if dot > dist {
                dist = dot;
                furthest = point;
            }
        }

        for vertex in &self.vertices {
            let dot: N64 = vertex.dot(&dir);
            if dot > dist {
                dist = dot;
                furthest = vertex.clone();
            }
        }

        furthest
    }
}