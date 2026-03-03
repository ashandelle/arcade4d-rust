use std::fmt;

use noisy_float::prelude::*;

use crate::{mathnd::{BiVecN, MatN, VecN}, physics::Collider};

#[derive(Debug, Clone)]
pub struct Material {
    pub restitution: N64,
}

#[derive(Debug, Clone)]
pub struct Momentum {
    pub linear: VecN,
    pub angular: BiVecN,
}

impl fmt::Display for Momentum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ linear: {}, angular: {}}}", self.linear, self.angular)
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub linear: VecN,
    pub angular: MatN,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ linear: {}, angular: {}}}", self.linear, self.angular)
    }
}

#[derive(Debug, Clone)]
pub enum Inertia {
    Scalar {s: N64},
    BiVec {b: BiVecN},
    // Tensor {t: BiMatN},
    Immovable,
}

#[derive(Clone)]
pub struct Body {
    pub mass: N64,
    pub inertia: Inertia,
    pub material: Material,
    pub stationary: bool,

    pub pos: Position,
    pub mom: Momentum,

    pub collider: Collider,
}

impl Body {
    pub fn resolve_impulse(
        &mut self,
        impulse: &VecN,
        world_contact: &VecN,
    ) {
        if !self.stationary {
            let body_contact = world_contact - &self.pos.linear;
            let delta_angular_mom = body_contact ^ impulse;

            self.mom.linear = &self.mom.linear + impulse;
            self.mom.angular = &self.mom.angular + &delta_angular_mom;
        }
    }

    pub fn step(&mut self, gravity: &VecN, dt: N64) {
        if !self.stationary {
            // apply gravity
            self.mom.linear = &self.mom.linear + gravity * (self.mass * dt);

            self.pos.linear = &self.pos.linear + &self.mom.linear / self.mass * dt;
            
            let mut angvelocity = self.body_bivec_to_world(&self.inverse_moment_of_inertia(&self.world_bivec_to_body(&self.mom.angular))) / self.mass;
            let rot = (&self.pos.angular + (angvelocity.to_matn() * &self.pos.angular) * (dt / 2.0)).orthonormalize();
            angvelocity = &rot * (&self.inverse_moment_of_inertia(&rot.mult_transpose_bivecn(&self.mom.angular))) / self.mass;
            self.pos.angular = (&self.pos.angular + (angvelocity.to_matn() * &self.pos.angular) * dt).orthonormalize();
        }
    }

    pub fn inverse_moment_of_inertia(&self, body_bivec: &BiVecN) -> BiVecN {
        match &self.inertia {
            Inertia::Scalar { s } => {
                body_bivec / *s
            },
            Inertia::BiVec { b } => {
                BiVecN {
                    m: MatN { 
                        e: (body_bivec.m.e).iter()
                            .zip((b.m.e).iter())
                            .map(|(x, y)| VecN {
                                e: x.e.iter()
                                    .zip(y.e.iter())
                                    .map(|(x, y)| *x / *y)
                                    .collect(),
                            })
                            .collect(),
                    },
                }
            },
            // Inertia::Tensor { t } => {
                
            // },
            Inertia::Immovable => {
                BiVecN::zero(body_bivec.m.e.len())
            },
        }
    }

    pub fn vel_at(&self, world_pos: &VecN) -> VecN {
        let body_pos = self.world_pos_to_body(&world_pos);

        let rot_vel = self.body_vec_to_world(
            &body_pos.left_contract(&self.inverse_moment_of_inertia(&self.world_bivec_to_body(&self.mom.angular)))
        );

        (&self.mom.linear + rot_vel) / self.mass
    }

    pub fn body_vec_to_world(&self, v: &VecN) -> VecN {
        &self.pos.angular * v
    }

    pub fn world_vec_to_body(&self, v: &VecN) -> VecN {
        self.pos.angular.mult_transpose(&v)
    }

    pub fn body_bivec_to_world(&self, v: &BiVecN) -> BiVecN {
        &self.pos.angular * v
    }

    pub fn world_bivec_to_body(&self, v: &BiVecN) -> BiVecN {
        self.pos.angular.mult_transpose_bivecn(&v)
    }

    pub fn body_pos_to_world(&self, v: &VecN) -> VecN {
        (&self.pos.angular * v) + &self.pos.linear
    }

    pub fn world_pos_to_body(&self, v: &VecN) -> VecN {
        self.pos.angular.mult_transpose(&(v - &self.pos.linear))
    }
}