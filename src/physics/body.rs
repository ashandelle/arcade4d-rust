use std::{fmt, iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub}};

use crate::physics::{Collider, Render};

use mathnd::{vecn::VecN, matn::MatN, bivecn::BiVecN, traits::{Abs, FromUsize, Sqrt, Two, Zero}};

#[derive(Debug, Clone)]
pub struct Material<T> {
    pub restitution: T,
}

#[derive(Debug, Clone)]
pub struct Velocity<T, const N: usize> {
    pub linear: VecN<T, N>,
    pub angular: BiVecN<T, N>,
}

// #[derive(Debug, Clone)]
// pub struct Momentum<T, const N: usize> {
//     pub linear: VecN<T, N>,
//     pub angular: BiVecN<T, N>,
// }

// impl fmt::Display for Momentum {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ linear: {}, angular: {}}}", self.linear, self.angular)
//     }
// }

#[derive(Debug, Clone)]
pub struct Position<T, const N: usize> {
    pub linear: VecN<T, N>,
    pub angular: MatN<T, N>,
}

// impl fmt::Display for Position {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ linear: {}, angular: {}}}", self.linear, self.angular)
//     }
// }

#[derive(Debug, Clone)]
pub enum Inertia<T, const N: usize> {
    Scalar {s: T},
    BiVec {b: BiVecN<T, N>},
    // Tensor {t: BiMatN},
    Immovable,
}

#[derive(Clone)]
pub struct Body<T, const N: usize> {
    pub mass: T,
    pub inertia: Inertia<T, N>,
    pub material: Material<T>,
    pub stationary: bool,

    pub pos: Position<T, N>,
    // pub mom: Momentum<T, N>,
    pub vel: Velocity<T, N>,

    pub collider: Collider<T, N>,
    pub render: Render<T, N>,
}

impl<T, const N: usize> Body<T, N> where T: 
Neg<Output = T> + Add<Output = T> + Sub<Output = T> +
Mul<Output = T> + Div<Output = T> +
AddAssign + DivAssign +
PartialOrd +
Sum +
Sqrt + Abs +
Zero + Two +
FromUsize +
Copy {
    pub fn resolve_impulse(
        &mut self,
        impulse: &VecN<T, N>,
        world_contact: &VecN<T, N>,
    ) {
        if !self.stationary {
            // let body_contact = *world_contact - self.pos.linear;
            // let delta_angular_mom = body_contact ^ *impulse;

            // self.mom.linear = self.mom.linear + *impulse;
            // self.mom.angular = self.mom.angular + delta_angular_mom;
            let body_contact = self.world_pos_to_body(world_contact);
            let delta_angular_vel = self.inverse_moment_of_inertia(
                &(body_contact ^ (self.pos.angular.transpose() * *impulse)),
            );

            self.vel.linear = self.vel.linear + *impulse / self.mass;
            self.vel.angular = self.vel.angular + delta_angular_vel;
        }
    }

    pub fn step(&mut self, gravity: &VecN<T, N>, eps: T, dt: T) {
        if !self.stationary {
            // apply gravity
            // self.mom.linear = self.mom.linear + (*gravity) * (self.mass * dt);
            self.vel.linear = self.vel.linear + (*gravity) * dt;

            // self.pos.linear = self.pos.linear + self.mom.linear / self.mass * dt;
            self.pos.linear = self.pos.linear + self.vel.linear * dt;
            
            // let mut angvelocity = self.body_bivec_to_world(&self.inverse_moment_of_inertia(&self.world_bivec_to_body(&self.mom.angular)));
            // let rot = (self.pos.angular + (angvelocity.to_matn() * self.pos.angular) * (dt / T::two())).orthonormalize(eps, 128);
            // angvelocity = rot * self.inverse_moment_of_inertia(&(rot.transpose() * self.mom.angular));
            // self.pos.angular = (self.pos.angular + (angvelocity.to_matn() * self.pos.angular) * dt).orthonormalize(eps, 128);
            let angvelocity = self.vel.angular;
            self.pos.angular = (self.pos.angular + (angvelocity.to_matn() * self.pos.angular) * dt).orthonormalize(eps, 128);
        }
    }

    pub fn inverse_moment_of_inertia(&self, body_bivec: &BiVecN<T, N>) -> BiVecN<T, N> {
        match &self.inertia {
            Inertia::Scalar { s } => {
                *body_bivec / (*s * self.mass)
            },
            Inertia::BiVec { b } => {
                // VecN {
                //     e: (body_bivec.to_vecn().e).iter()
                //         .zip(b.to_vecn().e.iter())
                //         .map(|(x,y)| *x / (*y * self.mass))
                //         .collect(),
                // }.to_bivecn()
                BiVecN {
                    m: MatN {
                        e: std::array::from_fn(|i| VecN {
                            e: std::array::from_fn::<T, _, _>(|j| body_bivec.m.e[i].e[j] / (b.m.e[i].e[j] * self.mass))
                        })
                    }
                }
            },
            // Inertia::Tensor { t } => {
                
            // },
            Inertia::Immovable => {
                BiVecN::zero()
            },
        }
    }

    pub fn vel_at(&self, world_pos: &VecN<T, N>) -> VecN<T, N> {
        if self.mass > T::zero() {
            // let body_pos = self.world_pos_to_body(&world_pos);

            // let rot_vel = self.body_vec_to_world(
            //     &body_pos.left_contract(self.inverse_moment_of_inertia(&self.world_bivec_to_body(&self.mom.angular)))
            // );

            // self.mom.linear / self.mass + rot_vel
            let body_pos = self.world_pos_to_body(world_pos);

            let rot_vel = self.body_vec_to_world(
                &body_pos.left_contract(self.vel.angular),
            );

            self.vel.linear + rot_vel
        } else {
            VecN::zero()
        }
    }

    pub fn body_vec_to_world(&self, v: &VecN<T, N>) -> VecN<T, N> {
        self.pos.angular * *v
    }

    pub fn world_vec_to_body(&self, v: &VecN<T, N>) -> VecN<T, N> {
        self.pos.angular.transpose() * *v
    }

    pub fn body_bivec_to_world(&self, v: &BiVecN<T, N>) -> BiVecN<T, N> {
        self.pos.angular * *v
    }

    pub fn world_bivec_to_body(&self, v: &BiVecN<T, N>) -> BiVecN<T, N> {
        self.pos.angular.transpose() * *v
    }

    pub fn body_pos_to_world(&self, v: &VecN<T, N>) -> VecN<T, N> {
        (self.pos.angular * *v) + self.pos.linear
    }

    pub fn world_pos_to_body(&self, v: &VecN<T, N>) -> VecN<T, N> {
        self.pos.angular.transpose() * (*v - self.pos.linear)
    }
}