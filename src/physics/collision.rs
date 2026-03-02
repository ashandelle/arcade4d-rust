use noisy_float::prelude::*;

use super::{Body, CollisionManifold};
use crate::mathnd::VecN;

#[derive(Debug)]
pub struct ContactState {
    contact: VecN,
    bias: N64,
    normal_mass: N64,
    normal_impulse: N64,
    tangent_mass: Vec<N64>,
    tangent_impulse: Vec<N64>,
}

pub struct CollisionConstraint {
    normal: VecN,
    tangents: Vec<VecN>,
    contacts: Vec<ContactState>,
    mu: N64,
}

impl CollisionConstraint {
    pub fn new(
        manifold: CollisionManifold,
        a: &Body,
        mass_adjustment_a: N64,
        b: &Body,
        mass_adjustment_b: N64,
    ) -> Self {
        let CollisionManifold {
            normal,
            depth,
            contacts,
        } = manifold;

        let dim = normal.e.len();

        let e = a.material.restitution.min(b.material.restitution);
        // TODO: move this into the Material struct
        let mu = n64(0.4);

        let tangents = normal.orthonormal_basis();

        let contacts: Vec<_> = contacts
            .into_iter()
            .map(|contact| {
                let rel_vel = b.vel_at(&contact) - a.vel_at(&contact);
                let rel_vel_normal = rel_vel.dot(&normal);

                let slop = n64(0.01);
                let baumgarte = n64(0.2);
                let bias = -baumgarte * 60.0 * (slop - depth).min(n64(0.0))
                    + if rel_vel_normal < -1.0 {
                        -e * rel_vel_normal
                    } else {
                        n64(0.0)
                    };

                let inv_a_mass = if a.mass > 0.0 {
                    mass_adjustment_a / a.mass
                } else {
                    n64(0.0)
                };
                let inv_b_mass = if b.mass > 0.0 {
                    mass_adjustment_b / b.mass
                } else {
                    n64(0.0)
                };

                let inverse_mass_term =
                    |body: &Body,
                     normal: &VecN,
                     contact: &VecN| {
                        // n' = ~R n R
                        let body_normal = body.world_vec_to_body(&normal);
                        let body_contact = body.world_pos_to_body(&contact);

                        // n . (R x . I_b^-1(x /\ n') ~R)
                        normal.dot(
                            &body.body_vec_to_world(
                                &body_contact.left_contract(
                                        &body.inverse_moment_of_inertia(&(&body_contact ^ body_normal))
                                    ),
                            ),
                        )
                    };

                let inv_l_a =
                    mass_adjustment_a * inverse_mass_term(a, &normal, &contact);
                let inv_l_b =
                    mass_adjustment_b * inverse_mass_term(b, &normal, &contact);

                let normal_mass =
                    n64(1.0) / (inv_a_mass + inv_b_mass + inv_l_a + inv_l_b);

                let mut tangent_mass = vec![n64(0.0); dim-1];
                for i in 0..(dim-1) {
                    let inv_l_t_a = mass_adjustment_a
                        * inverse_mass_term(a, &tangents[i], &contact);
                    let inv_l_t_b = mass_adjustment_b
                        * inverse_mass_term(b, &tangents[i], &contact);

                    tangent_mass[i] =
                        n64(1.0) / (inv_a_mass + inv_b_mass + inv_l_t_a + inv_l_t_b);
                }

                ContactState {
                    contact,
                    bias,
                    normal_mass,
                    normal_impulse: n64(0.0),
                    tangent_mass,
                    tangent_impulse: vec![n64(0.0); dim-1],
                }
            })
            .collect();

        Self {
            normal,
            tangents,
            contacts,
            mu,
        }
    }

    pub fn solve(&mut self, a: &mut Body, b: &mut Body) {
        for contact_state in self.contacts.iter_mut() {
            let ContactState {
                contact,
                bias,
                normal_mass,
                normal_impulse,
                tangent_mass,
                tangent_impulse,
            } = contact_state;

            let dim = contact.e.len();

            let rel_vel = b.vel_at(&contact) - a.vel_at(&contact);

            // calculate friction impulse
            let mut new_impulses = vec![n64(0.0); dim-1];
            for i in 0..(dim-1) {
                let lambda = -rel_vel.dot(&self.tangents[i]) * tangent_mass[i];
                new_impulses[i] = tangent_impulse[i] + lambda;
            }

            // clamp the total magnitude
            let max_impulse = (self.mu * *normal_impulse).abs();
            let mut impulse_mag2 = n64(0.0);
            new_impulses.iter().for_each(|i| impulse_mag2 += *i * i);
            let impulse_mag = impulse_mag2.sqrt();
            if impulse_mag > max_impulse {
                let factor = max_impulse / impulse_mag;
                new_impulses.iter_mut().for_each(|i| *i *= factor);
            }

            // apply the friction impulses
            for i in 0..(dim-1) {
                let impulse =
                    &self.tangents[i] * (new_impulses[i] - tangent_impulse[i]);
                tangent_impulse[i] = new_impulses[i];
                a.resolve_impulse(&-&impulse, contact);
                b.resolve_impulse(&impulse, contact);
            }

            // calculate normal impulse
            let rel_vel_normal = rel_vel.dot(&self.normal);
            let lambda = *normal_mass * (-rel_vel_normal + *bias);
            let prev_impulse = *normal_impulse;
            *normal_impulse = (prev_impulse + lambda).max(n64(0.0));
            let impulse = &self.normal * (*normal_impulse - prev_impulse);
            a.resolve_impulse(&-&impulse, contact);
            b.resolve_impulse(&impulse, contact);
        }
    }
}