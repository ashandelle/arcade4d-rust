use std::{iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use super::{Body, CollisionManifold};
use mathnd::{vecn::VecN, traits::{MinMax, Sqrt, Two}};
use num_traits::{FromPrimitive, One, Signed, Zero};

#[derive(Debug)]
pub struct ContactState<T, const N: usize> where [(); N-1]: Sized {
    contact: VecN<T, N>,
    bias: T,
    normal_mass: T,
    normal_impulse: T,
    tangent_mass: [T; N-1],
    tangent_impulse: [T; N-1],
}

pub struct CollisionConstraint<T, const N: usize> where [(); N-1]: Sized {
    normal: VecN<T, N>,
    tangents: [VecN<T, N>; N-1],
    contacts: Vec<ContactState<T, N>>,
    mu: T,
}

impl<T, const N: usize> CollisionConstraint<T, N> where T: 
Neg<Output = T> + Add<Output = T> + Sub<Output = T> +
Mul<Output = T> + Div<Output = T> +
AddAssign + SubAssign + MulAssign + DivAssign +
PartialOrd +
Sum +
Sqrt + Signed + MinMax +
Zero + One + Two +
FromPrimitive +
Copy,
[(); N-1]: Sized {
    pub fn new(
        manifold: CollisionManifold<T, N>,
        a: &Body<T, N>,
        mass_adjustment_a: T,
        b: &Body<T, N>,
        mass_adjustment_b: T,
    ) -> Self {
        let CollisionManifold {
            normal,
            depth,
            contacts,
        } = manifold;

        let e = a.material.restitution.min(b.material.restitution);
        // TODO: move this into the Material struct
        let mu = T::from_f32(0.4).unwrap();

        let tangents: [VecN<T, N>; N-1] = normal.orthonormal_basis();

        let contacts: Vec<_> = contacts
            .into_iter()
            .map(|contact| {
                let rel_vel = b.vel_at(&contact) - a.vel_at(&contact);
                let rel_vel_normal = rel_vel.dot(normal);

                let slop = T::from_f32(0.01).unwrap();
                let baumgarte = T::from_f32(0.2).unwrap();
                let bias = -baumgarte * T::from_f32(60.0).unwrap() * (slop - depth).min(T::zero())
                    + if rel_vel_normal < -T::one() {
                        -e * rel_vel_normal
                    } else {
                        T::zero()
                    };

                let inv_a_mass = if a.mass > T::zero() {
                    mass_adjustment_a / a.mass
                } else {
                    T::zero()
                };
                let inv_b_mass = if b.mass > T::zero() {
                    mass_adjustment_b / b.mass
                } else {
                    T::zero()
                };

                let inverse_mass_term =
                    |body: &Body<T, N>,
                     normal: &VecN<T, N>,
                     contact: &VecN<T, N>| {
                        // n' = ~R n R
                        let body_normal = body.world_vec_to_body(&normal);
                        let body_contact = body.world_pos_to_body(&contact);

                        // n . (R x . I_b^-1(x /\ n') ~R)
                        normal.dot(
                            body.body_vec_to_world(
                                &body_contact.left_contract(
                                        body.inverse_moment_of_inertia(&(body_contact ^ body_normal))
                                    ),
                            ),
                        )
                    };

                let inv_l_a =
                    mass_adjustment_a * inverse_mass_term(&a, &normal, &contact);
                let inv_l_b =
                    mass_adjustment_b * inverse_mass_term(&b, &normal, &contact);

                let normal_mass =
                    T::one() / (inv_a_mass + inv_b_mass + inv_l_a + inv_l_b);

                let mut tangent_mass = [T::zero(); N-1];
                for i in 0..(N-1) {
                    let inv_l_t_a = mass_adjustment_a
                        * inverse_mass_term(&a, &tangents[i], &contact);
                    let inv_l_t_b = mass_adjustment_b
                        * inverse_mass_term(&b, &tangents[i], &contact);

                    tangent_mass[i] =
                        T::one() / (inv_a_mass + inv_b_mass + inv_l_t_a + inv_l_t_b);
                }

                ContactState {
                    contact,
                    bias,
                    normal_mass,
                    normal_impulse: T::zero(),
                    tangent_mass,
                    tangent_impulse: [T::zero(); N-1],
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

    pub fn solve(&mut self, a: &mut Body<T, N>, b: &mut Body<T, N>) {
        for contact_state in self.contacts.iter_mut() {
            let ContactState {
                contact,
                bias,
                normal_mass,
                normal_impulse,
                tangent_mass,
                tangent_impulse,
            } = contact_state;

            let rel_vel = b.vel_at(&contact) - a.vel_at(&contact);

            // calculate friction impulse
            let mut new_impulses = vec![T::zero(); N-1];
            for i in 0..(N-1) {
                let lambda = -rel_vel.dot(self.tangents[i]) * tangent_mass[i];
                new_impulses[i] = tangent_impulse[i] + lambda;
            }

            // clamp the total magnitude
            let max_impulse = (self.mu * *normal_impulse).abs();
            let mut impulse_mag2 = T::zero();
            new_impulses.iter().for_each(|i| impulse_mag2 += *i * *i);
            let impulse_mag = impulse_mag2.sqrt();
            if impulse_mag > max_impulse {
                let factor = max_impulse / impulse_mag;
                new_impulses.iter_mut().for_each(|i| *i *= factor);
            }

            // apply the friction impulses
            for i in 0..(N-1) {
                let impulse =
                    self.tangents[i] * (new_impulses[i] - tangent_impulse[i]);
                tangent_impulse[i] = new_impulses[i];
                a.resolve_impulse(&-impulse, &contact);
                b.resolve_impulse(&impulse, &contact);
            }

            // calculate normal impulse
            let rel_vel_normal = rel_vel.dot(self.normal);
            let lambda = *normal_mass * (-rel_vel_normal + *bias);
            let prev_impulse = *normal_impulse;
            *normal_impulse = (prev_impulse + lambda).max(T::zero());
            let impulse = self.normal * (*normal_impulse - prev_impulse);
            a.resolve_impulse(&-impulse, &contact);
            b.resolve_impulse(&impulse, &contact);
        }
    }
}