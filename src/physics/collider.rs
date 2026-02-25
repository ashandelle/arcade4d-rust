use crate::mathnd::{VecN, BiVecN, MatN};

#[derive(Clone)]
pub enum Collider {
    HalfSpace { normal: VecN },
    Polytope { poly: Polytope }, // Convex only
    // Mesh { mesh: Mesh },
    Sphere { radius: f64 },
}

#[derive(Debug)]
pub struct CollisionManifold {
    pub normal: VecN,
    pub depth: f64,
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