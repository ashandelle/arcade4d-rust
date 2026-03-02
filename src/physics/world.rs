use std::collections::HashMap;

use crate::physics::{Body, CollisionConstraint, CollisionDetection};

pub struct Object {
    pub body: Body,
    // pub renderer: Option<MeshBinding>,
}

pub struct World {
    pub objects: Vec<Object>,
    pub collision: CollisionDetection,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            collision: CollisionDetection::new(),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut collisions = Vec::new();
        let mut mass_adjustments = HashMap::new();

        for i in 0..self.objects.len() {
            for j in i + 1..self.objects.len() {
                let a = &self.objects[i];
                let b = &self.objects[j];

                if let Some(manifold) =
                    self.collision.detect_collisions((i, j), &a.body, &b.body)
                {
                    if manifold.contacts.len() == 0 {
                        continue;
                    }
                    *mass_adjustments.entry(i).or_insert(0) += 1;
                    *mass_adjustments.entry(j).or_insert(0) += 1;
                    collisions.push((i, j, manifold));
                }
            }
        }

        let mut constraints = Vec::new();
        for (i, j, manifold) in collisions {
            constraints.push((
                i,
                j,
                CollisionConstraint::new(
                    manifold,
                    &self.objects[i].body,
                    mass_adjustments[&i] as f64,
                    &self.objects[j].body,
                    mass_adjustments[&j] as f64,
                ),
            ));
        }

        const SOLVER_ITERS: usize = 20;
        for _ in 0..SOLVER_ITERS {
            for (i, j, constraint) in constraints.iter_mut() {
                let ms = self.objects.split_at_mut(*j);
                let (a, b) = (&mut ms.0[*i], &mut ms.1[0]);
                constraint.solve(&mut a.body, &mut b.body);
            }
        }

        for object in &mut self.objects {
            object.body.step(dt);
        }
    }

    // pub fn render<'a: 'c, 'b, 'c>(
    //     &'a self,
    //     pipeline: &'a TriangleListPipeline,
    //     render_pass: &'b mut wgpu::RenderPass<'c>,
    // ) {
    //     for i in self.objects.values() {
    //         i.render(pipeline, render_pass);
    //     }
    // }
}