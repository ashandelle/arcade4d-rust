use std::{iter::Sum, marker::PhantomData, ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub}};

use crate::physics::{Body, Polytope};

use mathnd::{vecn::VecN, traits::{Sqrt, Two}};
use num_traits::{Bounded, FromPrimitive, One, Signed, Zero};

#[derive(Clone)]
pub enum Collider<T, const N: usize> {
    HalfSpace { normal: VecN<T, N> },
    Sphere { radius: T },
    // Rotatope { rota: Rotatope },
    // Tegum { parts: Vec<Collider> },
    // Prism { parts: Vec<Collider> },
    // Minkowski { parts: Vec<Collider> },
    Polytope { maxradius: T, poly: Polytope<T, N> }, // Convex only
    // Mesh { mesh: Mesh },
}

#[derive(Debug)]
pub struct CollisionManifold<T, const N: usize> {
    pub normal: VecN<T, N>,
    pub depth: T,
    pub contacts: Vec<VecN<T, N>>,
}

// #[derive(Copy, Clone)]
// pub struct MeshRef<'a> {
//     pub body: &'a Body,
//     pub mesh: &'a Mesh,
// }

// #[derive(Copy, Clone)]
// pub struct SphereRef<'a> {
//     pub body: &'a Body,
//     pub radius: f32,
// }

// #[derive(Debug)]
// struct VertexCellContact {
//     // if true indicates that the vertex is on body b but the cell is on body a
//     side: bool,
//     vertex_idx: usize,
//     cell_idx: usize,
//     normal: Vector4<f32>,
// }

// #[derive(Debug)]
// struct EdgeFaceContact {
//     // if true indicates that the edge is on body b but the face is on body a
//     side: bool,
//     k: Vector4<f32>,
//     t: Vector4<f32>,
//     s: Vector4<f32>,
//     u: Vector4<f32>,
//     v: Vector4<f32>,
//     normal: Vector4<f32>,
// }

// #[derive(Debug)]
// enum ContactData {
//     VertexCell(VertexCellContact),
//     EdgeFace(EdgeFaceContact),
// }

pub struct CollisionDetection<T, const N: usize> {
    numbertype: PhantomData<T>
}

impl<T, const N: usize> CollisionDetection<T, N> where T: 
Neg<Output = T> + Add<Output = T> + Sub<Output = T> +
Mul<Output = T> + Div<Output = T> +
AddAssign + DivAssign +
PartialOrd +
Sum +
Sqrt + Signed +
Zero + One + Two + Bounded +
FromPrimitive +
Copy {
    pub fn new() -> Self {
        Self {
            numbertype: PhantomData,
        }
    }

    pub fn detect_collisions(
        &mut self,
        key: (usize, usize),
        a: &Body<T, N>,
        b: &Body<T, N>,
    ) -> Option<CollisionManifold<T, N>> {
        match (&a.collider, &b.collider) {
            (Collider::HalfSpace { normal }, Collider::Sphere { radius }) => {
                let plane_distance = a.pos.linear.dot(*normal);
                let sphere_distance = b.pos.linear.dot(*normal);

                let center_distance = sphere_distance - plane_distance;
                if center_distance < *radius {
                    Some(CollisionManifold {
                        normal: normal.clone(),
                        depth: *radius - center_distance,
                        contacts: vec![b.pos.linear - *normal * *radius],
                    })
                } else {
                    None
                }
            }
            (Collider::Sphere { .. }, Collider::HalfSpace { .. }) => {
                // Just call this again with the arguments swapped
                let mut manifold = self.detect_collisions((key.1, key.0), b, a);
                if let Some(m) = &mut manifold {
                    m.normal = -m.normal;
                }
                manifold
            }
            (
                Collider::Sphere { radius: radius_a },
                Collider::Sphere { radius: radius_b },
            ) => {
                let displacement = b.pos.linear - a.pos.linear;
                let depth = *radius_a + *radius_b - displacement.length();
                if depth > T::zero() {
                    let normal = displacement.normalized();
                    Some(CollisionManifold {
                        // contacts: vec![&a.pos.linear + depth * &normal],
                        contacts: vec![a.pos.linear + normal * (*radius_a - depth / T::two())],
                        normal,
                        depth,
                    })
                } else {
                    None
                }
            }
            (Collider::HalfSpace { normal }, Collider::Polytope { maxradius, poly }) => {
                let sphere_distance = b.pos.linear.dot(*normal);
                if sphere_distance > *maxradius {
                    return None;
                }

                let supports = poly.support(&-b.world_vec_to_body(normal));
                let p: [VecN<T, N>; N] = std::array::from_fn(|i| b.body_pos_to_world(&supports[i]));
                let dist = normal.dot(p[0]);



                if dist < T::zero() {
                    Some(CollisionManifold {
                        normal: *normal,
                        depth: -dist,
                        contacts: p.iter().filter_map(|point| {
                            let dist = normal.dot(*point);
                            if dist < T::zero() {
                                Some(*point)
                            } else {
                                None
                            }
                        }).collect(),
                    })
                } else {
                    None
                }
            }
            (Collider::Polytope { .. }, Collider::HalfSpace { .. }) => {
                // Just call this again with the arguments swapped
                let mut manifold = self.detect_collisions((key.1, key.0), b, a);
                if let Some(m) = &mut manifold {
                    m.normal = -m.normal;
                }
                manifold
            }
            _ => None,
        }
    }
}