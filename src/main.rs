use std::time::Instant;

use crate::mathnd::{BiVecN, MatN, VecN};
use crate::physics::{Body, Collider, Inertia, Material, Momentum, Object, Position, World};

use noisy_float::prelude::*;
use rand::Rng;
use rand_distr::StandardNormal;

mod mathnd;
mod physics;

fn main() {
    let mut rng = rand::rng();

    let dim = 5;

    let sec = 1;
    let step = 100;
    let dt = n64(1.0) / n64(step as f64);

    let mut world = World::new(dim);
    world.gravity = n64(-9.8) * VecN::basis(dim, 0);

    world.objects.push(Object {
        body: Body {
            mass: n64(1.0),
            inertia: Inertia::Immovable,
            stationary: true,
            pos: Position {
                linear: VecN::zero(dim),
                angular: MatN::identity(dim),
            },
            mom: Momentum {
                linear: VecN::zero(dim),
                angular: BiVecN::zero(dim),
            },
            collider: Collider::HalfSpace {
                normal: VecN::basis(dim, 0),
            },
            material: Material {
                restitution: n64(0.4),
            },
        },
    });

    for i in 0..100 {
        world.objects.push(Object {
            body: Body {
                mass: n64(1.0),
                inertia: Inertia::Scalar { s: n64(1.0) },
                stationary: false,
                pos: Position {
                    linear: n64(4.0 * (i+1) as f64) * VecN::basis(dim, 0),
                    angular: MatN::identity(dim),
                },
                mom: Momentum {
                    linear: VecN {
                        e: (0..dim).map(|_x| n64(0.01) * n64(rng.sample(StandardNormal))).collect()
                    },
                    angular: VecN {
                        e: (0..((dim*dim - dim) / 2)).map(|_x| n64(0.01) * n64(rng.sample(StandardNormal))).collect()
                    }.to_bivecn(),
                },
                collider: Collider::Sphere { radius: n64(1.0) },
                material: Material {
                    restitution: n64(0.4),
                },
            },
        });
    }

    let start = Instant::now();

    let mut i = 0;
    while i < sec*step {
        world.update(dt);

        for j in 0..world.objects.len() {
            let obj = &world.objects[j];
            println!("{}", obj.body.pos);
            println!("{}", obj.body.mom);
        }

        i+=1;
    }

    let duration = start.elapsed();

    println!("{:?}", duration);
}