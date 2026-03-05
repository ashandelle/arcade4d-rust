use noisy_float::prelude::*;

use crate::{mathnd::VecN, physics::Body};

#[derive(Clone)]
pub enum Render {
    HalfSpace { normal: VecN },
    Sphere { radius: N64 },
    Box { dimensions: VecN },
    Orthoplex { radius: N64 },
}

impl Render {
    pub fn sdf(&self, body: &Body, vec: &VecN) -> N64 {
        match self {
            Self::HalfSpace { normal } => {
                (vec - &body.pos.linear).dot(normal)
            },
            Self::Sphere { radius } => {
                (vec - &body.pos.linear).length() - radius
            },
            Self::Box { dimensions } => {
                let mut p = body.world_pos_to_body(vec);
                p.e = (p.e).iter()
                    .zip((dimensions.e).iter())
                    .map(|(x,y)| x.abs() - y)
                    .collect();
                let m2 = match (p.e).iter().max() {
                    Some(value) => *value,
                    None => n64(0.0),
                }.min(n64(0.0));
                p.e = (p.e).iter().map(|x| *x.max(&n64(0.0))).collect();
                p.length() + m2
            },
            Self::Orthoplex { radius } => {
                let p = body.world_pos_to_body(vec);
                let sum = (p.e).iter()
                    .map(|x| x.abs())
                    .sum::<N64>();
                (sum - radius) / n64(vec.e.len() as f64).sqrt()
            },
        }
    }
}