use crate::mathnd::{VecN, BiVecN, MatN};

#[derive(Debug, Clone)]
pub struct Material {
    pub restitution: f64,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub linear: VecN,
    pub angular: BiVecN,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub linear: VecN,
    pub angular: MatN,
}

#[derive(Debug, Clone)]
pub enum Inertia {
    Scalar {s: f64},
    BiVec {b: BiVecN},
    // Tensor {t: BiMatN},
    Immovable,
}

#[derive(Clone)]
pub struct Body {
    pub mass: f64,
    pub inertia: Inertia,
    pub material: Material,
    pub stationary: bool,

    pub pos: Position,
    pub vel: Velocity,

    pub collider: Collider,
}

impl Body {
    // pub fn resolve_impulse(
    //     &mut self,
    //     impulse: VecN,
    //     world_contact: VecN,
    // ) {
    //     if !self.stationary {
    //         let body_contact = self.world_pos_to_body(world_contact);
    //         let delta_angular_vel = self.inverse_moment_of_inertia(
    //             &Vec4::from(body_contact)
    //                 .wedge_v(&self.rotation.reverse().rotate(&impulse.into())),
    //         );

    //         self.vel.linear += impulse / self.mass;
    //         self.vel.angular = self.vel.angular + delta_angular_vel;
    //     }
    // }

    pub fn step(&mut self, dt: f64) {
        if !self.stationary {
            // apply gravity
            // self.vel.linear += VecN::unit_y() * (-9.8 * dt);

            self.pos.linear = self.pos.linear + self.vel.linear * dt;
            self.pos.angular.update(&(dt * self.vel.angular));
        }
    }

    pub fn inverse_moment_of_inertia(&self, body_bivec: &BiVecN) -> BiVecN {
        match self.inertia {
            Inertia::Scalar { s } => {
                body_bivec / s
            },
            Inertia::BiVec { b } => {
                BiVecN {
                    ee: (body_bivec.ee).iter()
                        .zip((b.ee).iter())
                        .map(|(x, y)| x / y)
                        .collect(),
                }
            },
            // Inertia::Tensor { t } => {
                
            // },
            Inertia::Immovable => {
                BiVecN::zero()
            },
        }
    }

    pub fn vel_at(&self, world_pos: VecN) -> VecN {
        let body_pos = self.world_pos_to_body(world_pos);

        let rot_vel = self.body_vec_to_world(
            VecN::from(body_pos)
                .left_contract_bv(&self.vel.angular)
                .into(),
        );

        self.vel.linear + rot_vel
    }

    // pub fn ray_intersect(
    //     &self,
    //     start: Vector4<f32>,
    //     dir: Vector4<f32>,
    // ) -> Option<f32> {
    //     let start = self.world_pos_to_body(start);
    //     let dir = self.world_vec_to_body(dir);

    //     match &self.collider {
    //         Collider::Mesh { mesh } => {
    //             let mut interval = (std::f32::NEG_INFINITY, std::f32::INFINITY);

    //             for cell in mesh.cells.iter() {
    //                 // grab a representative vertex on the cell
    //                 let v0 = mesh.vertices[mesh.edges
    //                     [mesh.faces[cell.faces[0]].edges[0]]
    //                     .hd_vertex];

    //                 let denom = dir.dot(cell.normal);
    //                 let lambda = (v0 - start).dot(cell.normal) / denom;

    //                 if denom < 0.0 {
    //                     interval.0 = interval.0.max(lambda);
    //                 } else {
    //                     interval.1 = interval.1.min(lambda);
    //                 }

    //                 if interval.1 < interval.0 {
    //                     return None;
    //                 }
    //             }

    //             Some(interval.0)
    //         }
    //         Collider::Sphere { radius } => {
    //             // Solve a quadratic equation!
    //             let a = dir.magnitude2();
    //             let b = 2.0 * start.dot(dir);
    //             let c = start.magnitude2() - radius * radius;

    //             let discriminant = b * b - 4.0 * a * c;
    //             if discriminant >= 0.0 {
    //                 Some((-b - discriminant.sqrt()) / (2.0 * a))
    //             } else {
    //                 None
    //             }
    //         }
    //         _ => None,
    //     }
    // }

    pub fn body_vec_to_world(&self, v: VecN) -> VecN {
        self.pos.angular * v
    }

    pub fn world_vec_to_body(&self, v: VecN) -> VecN {
        self.pos.angular.transpose_mult(v);
    }

    pub fn body_pos_to_world(&self, v: VecN) -> VecN {
        (self.pos.angular * v) + self.pos.linear
    }

    pub fn world_pos_to_body(&self, v: VecN) -> VecN {
        self.pos.angular.transpose_mult(v - self.pos.linear)
    }
}