mod bivecn;
mod matn;
mod vecn;

pub use bivecn::BiVecN;
pub use matn::MatN;
pub use vecn::VecN;

// Gets a vector that's perpendicular to all vectors given.
// pub fn orthogonal_product(
//     u: Vector4<f32>,
//     v: Vector4<f32>,
//     w: Vector4<f32>,
// ) -> Vector4<f32> {
//     let u: Vec4 = u.into();
//     u.wedge_v(&v.into())
//         .wedge_v(&w.into())
//         .mul_qv(&Quadvec4::one())
//         .into()
// }