use std::{collections::HashMap, iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use crate::physics::{Body, CollisionConstraint, CollisionDetection};

use mathnd::{vecn::VecN, traits::{MinMax, Sqrt, Two}};
use num_traits::{Bounded, FromPrimitive, One, Signed, Zero};

pub struct Object<T, const N: usize> {
    pub body: Body<T, N>,
    // pub renderer: Option<MeshBinding>,
}

pub struct World<T, const N: usize> {
    pub objects: Vec<Object<T, N>>,
    pub collision: CollisionDetection<T, N>,

    pub eps: T,

    pub gravity: VecN<T, N>,
}

impl<T, const N: usize> World<T, N> where T: 
Neg<Output = T> + Add<Output = T> + Sub<Output = T> +
Mul<Output = T> + Div<Output = T> +
AddAssign + SubAssign + DivAssign + MulAssign +
PartialOrd + MinMax +
Sum +
Sqrt + Signed +
Zero + One + Two + Bounded +
FromPrimitive +
Copy,
[(); N-1]: Sized {
    pub fn new(eps: T) -> Self {
        Self {
            objects: Vec::new(),
            collision: CollisionDetection::new(),
            gravity: VecN::zero(),
            eps: eps,
        }
    }

    pub fn update(&mut self, dt: T) {
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
                    T::from_i32(mass_adjustments[&i]).unwrap(),
                    &self.objects[j].body,
                    T::from_i32(mass_adjustments[&j]).unwrap(),
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
            object.body.step(&self.gravity, self.eps, dt);
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