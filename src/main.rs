#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

// use crate::mathnd::{BiVecN, MatN, VecN};
use crate::mathnd::*;
use crate::physics::{Body, Collider, Inertia, Material, Velocity, Object, Polytope, Position, Render, World};

use image::{ImageBuffer, Rgb, RgbImage};
use noisy_float::prelude::*;
use rand::Rng;
use rand_distr::StandardNormal;

mod mathnd;
mod physics;
mod n64traits;

type Num = f64;
const DIM: usize = 3;
type Vector = VecN<Num, DIM>;
type BiVec = BiVecN<Num, DIM>;
type Mat = MatN<Num, DIM>;

fn main() {
    let mut rng = rand::rng();

    let sec = 5;
    let step = 60;
    let dt: Num = Num::fromf64(1.0 / (step as f64));

    let mut colors: Vec<(f64,f64,f64)> = Vec::new();

    for _i in 0..(std::cmp::min(std::cmp::max(1 << DIM, 2*DIM), 1 << 16)) {
        colors.push((rng.random(),rng.random(),rng.random()));
    }

    let mut light: Vector = Vector {
                        // e: (0..dim).map(|_x| n64(rng.sample(StandardNormal))).collect()
                        e: std::array::from_fn(|i| Num::fromf64(rng.sample(StandardNormal))),
                    };
    light.e[0] = Num::fromf32(-4.0);
    light.e[1] *= (DIM/2) as f64;
    light.e[DIM-1] *= (DIM/2) as f64;
    light = light.normalize();

    let mut world: World<Num, DIM> = World::new(Num::fromf64(1e-8));
    world.gravity = Vector::basis(0) * Num::fromf64(-9.8);

    world.objects.push(Object {
        body: Body {
            mass: Num::zero(),
            inertia: Inertia::Immovable,
            stationary: true,
            pos: Position {
                linear: Vector::zero(),
                angular: MatN::identity(),
            },
            // mom: Momentum {
            //     linear: Vector::zero(),
            //     angular: BiVecN::zero(),
            // },
            vel: Velocity {
                linear: Vector::zero(),
                angular: BiVecN::zero(),
            },
            collider: Collider::HalfSpace {
                normal: Vector::basis(0),
            },
            render: Render::HalfSpace {
                normal: Vector::basis(0),
            },
            material: Material {
                restitution: Num::fromf32(0.4),
            },
        },
    });

    // for i in 0..10 {
    //     world.objects.push(Object {
    //         body: Body {
    //             mass: n64(1.0),
    //             inertia: Inertia::Scalar { s: n64(1.0) },
    //             stationary: false,
    //             pos: Position {
    //                 linear: n64(3.0 * (i+1) as f64) * VecN::basis(dim, 0),
    //                 angular: MatN::identity(dim),
    //             },
    //             mom: Momentum {
    //                 linear: VecN {
    //                     e: (0..dim).map(|_x| n64(0.01) * n64(rng.sample(StandardNormal))).collect()
    //                 },
    //                 angular: BiVecN::basis(dim, 0, 1) + VecN {
    //                     e: (0..((dim*dim - dim) / 2)).map(|_x| n64(0.01) * n64(rng.sample(StandardNormal))).collect()
    //                 }.to_bivecn(),
    //             },
    //             collider: Collider::Sphere { radius: n64(1.0) },
    //             render: Render::Sphere { radius: n64(1.0) },
    //             material: Material {
    //                 restitution: n64(0.4),
    //             },
    //         },
    //     });
    // }

    world.objects.push(Object {
        body: Body {
            mass: Num::fromf32(1.0),
            inertia: Inertia::Scalar { s: Num::fromf32(1.0) },
            stationary: false,
            pos: Position {
                linear: VecN::basis(0) * Num::fromf32(4.0),
                angular: MatN {
                    e: std::array::from_fn(|i|
                        Vector::new(std::array::from_fn(|i| Num::fromf64(rng.sample(StandardNormal))))
                    ),
                }.orthonormalize(Num::fromf64(1e-8), 128),
            },
            vel: Velocity {
                // linear: VecN {
                //     e: (0..dim).map(|_x| n64(0.01) * n64(rng.sample(StandardNormal))).collect()
                // },
                linear: VecN::zero(),
                angular: BiVecN {
                    m: MatN {
                        e: std::array::from_fn(|i|
                            Vector::new(std::array::from_fn(|i| Num::fromf64(rng.sample(StandardNormal))))
                        ),
                    },
                }.skew() * Num::fromf64(0.05) + BiVecN::basis(0, 1),
            },
            collider: Collider::Polytope { maxradius: Num::fromf32(1.0), poly: Polytope::octahedron() },
            render: Render::Orthoplex { radius: Num::fromf32(1.0) },
            material: Material {
                restitution: Num::fromf32(0.4),
            },
        },
    });

    let start = Instant::now();

    // let mut file = match File::create("output.txt") {
    //     Ok(value) => value,
    //     Err(error) => {
    //         eprintln!("Problem opening the file: {:?}", error);
    //         panic!("Cannot proceed without a file")
    //     },
    // };

    let mut i = 0;
    while i < sec*step {
        world.update(dt);

        // for j in 0..(&world.objects).len() {
        //     let obj = &world.objects[j];
        //     // println!("{}", obj.body.pos);
        //     // println!("{}", obj.body.mom);
        //     // writeln!(&mut file, "{}", obj.body.pos);
        // }

        render(i, &colors, &light, &world.objects);
        println!("{}", i);

        i+=1;
    }

    let duration = start.elapsed();

    println!("{:?}", duration);
}

// ffmpeg -framerate 60 -i output_%03d.png -c:v libx264 -pix_fmt yuv420p output.mp4
fn render(n: usize, colors: &Vec<(f64,f64,f64)>, light: &Vector, objects: &Vec<Object<Num, DIM>>) {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Create a new ImageBuffer with the specified dimensions and pixel type (Rgb in this case)
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let pos = VecN::basis(0) * Num::fromf32(2.0) + VecN::basis(DIM-1) * Num::fromf32(-4.0);

    let MAX_STEPS = 500;
    let MAX_DIST = 1e4;
    let EPS = 1e-4;

    let mut culled = Vec::new();

    for object in objects {
        match &object.body.collider {
            Collider::HalfSpace { normal: _normal } => {
                culled.push(object);
            },
            Collider::Sphere { radius } => {
                let mut center = object.body.pos.linear - pos;
                center.e[0] = Num::zero();
                center.e[1] = Num::zero();
                center.e[DIM-1] = Num::zero();
                if center.length_sqr() < *radius*radius {
                    culled.push(object);
                }
            },
            Collider::Polytope { maxradius, poly: _poly } => {
                let mut center = object.body.pos.linear - pos;
                center.e[0] = Num::zero();
                center.e[1] = Num::zero();
                center.e[DIM-1] = Num::zero();
                if center.length_sqr() < *maxradius*maxradius {
                    culled.push(object);
                }
            },
        }
    }

    // Iterate over the pixels and set their color values
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let fx: Num = Num::fromf64(2.0 * (x as f64 / (IMAGE_WIDTH - 1) as f64) - 1.0);
        let fy: Num = Num::fromf64(-2.0 * (y as f64 / (IMAGE_HEIGHT - 1) as f64) + 1.0);

        // Calculate RGB values based on coordinates
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        let vec = (VecN::basis(0) * fy + VecN::basis(1) * fx + VecN::basis(DIM-1)).normalize();
        let mut t: Num = Num::zero();

        for _i in 0..MAX_STEPS {
            let loc: Vector = pos + vec * t;

            let mut dist: Num = MinMaxValue::maximum();
            let mut id = culled.len();

            for j in 0..culled.len() {
                // let d = match &culled[j].body.collider {
                //     Collider::HalfSpace { normal } => {
                //         (&loc - &culled[j].body.pos.linear).dot(normal)
                //     },
                //     Collider::Sphere { radius } => {
                //         (&loc - &culled[j].body.pos.linear).length() - radius
                //     },
                //     Collider::Polytope { maxradius: _maxradius, poly } => {
                //         (&loc - poly.nearest_point(&culled[j].body.world_pos_to_body(&loc))).length()
                //     },
                // };
                let d = culled[j].body.render.sdf(&culled[j].body, &loc);
                if d < dist {
                    dist = d;
                    id = j;
                }
            }

            if dist < EPS {
                let mut v = [Num::zero(); DIM];
                for i in (0..DIM) {
                    let mut dloc = loc.clone();
                    dloc.e[i] = dloc.e[i] + EPS * 1e-2;
                    v[i] = culled[id].body.render.sdf(&culled[id].body, &dloc) - dist;
                }
                let norm = VecN {
                    e: v,
                }.normalize();

                (r,g,b) = match &culled[id].body.collider {
                    Collider::HalfSpace { normal: _normal } => {
                        let s: i32 = loc.e.iter()
                            .map(|x| Num::toi32(x.round()))
                            .sum();
                        if s % 2 == 0 {
                            (0.25,0.25,0.25)
                        } else {
                            (0.75,0.75,0.75)
                        }
                    },
                    Collider::Sphere { radius: _radius } => {
                        let c = loc - culled[id].body.pos.linear;
                        let mut n = 0;
                        for j in 0..DIM {
                            if c.dot(culled[id].body.pos.angular.e[j]) >= 0.0 {
                                n += 1<<j;
                            }
                        }
                        colors[n]
                    },
                    Collider::Polytope { maxradius: _maxradius, poly: _poly } => {
                        (0.5, 0.5, 0.5)
                    },
                };
                
                let mut f = 10.0 / (t.tof64() + 10.0);
                f *= (norm.dot(*light).tof64() - 3.0) / -4.0;

                r *= f;
                g *= f;
                b *= f;
                break;
            }

            if dist > MAX_DIST {
                break;
            }

            t += dist;
        }

        // Convert f64 to u8 (0-255 range)
        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        // Assign the new color to the pixel
        *pixel = Rgb([ir, ig, ib]);
    }

    // Save the buffer to a file (format is inferred from the extension, e.g., "image.png")
    let string = format!("images/output_{}.png", n);
    if let Err(e) = buffer.save(&Path::new(&string)) {
        eprintln!("Error saving image: {}", e);
    // } else {
        // println!("Image saved successfully as output.png");
    }
}