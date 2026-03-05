use noisy_float::prelude::*;

use crate::{mathnd::VecN, physics::{Body, Polytope}};

#[derive(Clone)]
pub enum Collider {
    HalfSpace { normal: VecN },
    Sphere { radius: N64 },
    // Rotatope { rota: Rotatope },
    // Tegum { parts: Vec<Collider> },
    // Prism { parts: Vec<Collider> },
    // Minkowski { parts: Vec<Collider> },
    Polytope { maxradius: N64, poly: Polytope }, // Convex only
    // Mesh { mesh: Mesh },
}

#[derive(Debug)]
pub struct CollisionManifold {
    pub normal: VecN,
    pub depth: N64,
    pub contacts: Vec<VecN>,
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

pub struct CollisionDetection {
}

impl CollisionDetection {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn detect_collisions(
        &mut self,
        key: (usize, usize),
        a: &Body,
        b: &Body,
    ) -> Option<CollisionManifold> {
        match (&a.collider, &b.collider) {
            (Collider::HalfSpace { normal }, Collider::Sphere { radius }) => {
                let plane_distance = a.pos.linear.dot(normal);
                let sphere_distance = b.pos.linear.dot(normal);

                let center_distance = sphere_distance - plane_distance;
                if center_distance < *radius {
                    Some(CollisionManifold {
                        normal: normal.clone(),
                        depth: *radius - center_distance,
                        contacts: vec![&b.pos.linear - *radius * normal],
                    })
                } else {
                    None
                }
            }
            (Collider::Sphere { .. }, Collider::HalfSpace { .. }) => {
                // Just call this again with the arguments swapped
                let mut manifold = self.detect_collisions((key.1, key.0), b, a);
                if let Some(m) = &mut manifold {
                    m.normal = -&m.normal;
                }
                manifold
            }
            (
                Collider::Sphere { radius: radius_a },
                Collider::Sphere { radius: radius_b },
            ) => {
                let displacement = &b.pos.linear - &a.pos.linear;
                let depth = *radius_a + *radius_b - displacement.length();
                if depth > 0.0 {
                    let normal = displacement.normalize();
                    Some(CollisionManifold {
                        contacts: vec![&a.pos.linear + depth * &normal],
                        normal,
                        depth,
                    })
                } else {
                    None
                }
            }
            (Collider::HalfSpace { normal }, Collider::Polytope { maxradius, poly }) => {
                let plane_distance = a.pos.linear.dot(normal);
                let sphere_distance = b.pos.linear.dot(normal);

                let center_distance = sphere_distance - plane_distance;
                if center_distance > *maxradius {
                    return None;
                }

                let p = b.body_pos_to_world(&poly.support(&-b.world_vec_to_body(normal)));
                let dist = normal.dot(&p);

                if dist < 0.0 {
                    Some(CollisionManifold {
                        normal: normal.clone(),
                        depth: -dist,
                        contacts: vec![p],
                    })
                } else {
                    None
                }
            }
            (Collider::Polytope { .. }, Collider::HalfSpace { .. }) => {
                // Just call this again with the arguments swapped
                let mut manifold = self.detect_collisions((key.1, key.0), b, a);
                if let Some(m) = &mut manifold {
                    m.normal = -&m.normal;
                }
                manifold
            }
            _ => None,
        }
    }
}