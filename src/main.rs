use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::mathnd::{BiVecN, MatN, VecN};
use crate::physics::{Body, Collider, Inertia, Material, Momentum, Object, Position, World};

use image::{ImageBuffer, Rgb, RgbImage};
use noisy_float::prelude::*;
use rand::Rng;
use rand_distr::StandardNormal;

mod mathnd;
mod physics;

fn main() {
    let mut rng = rand::rng();

    let dim = 10;

    let sec = 10;
    let step = 60;
    let dt = n64(1.0) / n64(step as f64);

    let mut colors: Vec<(f64,f64,f64)> = Vec::new();

    for _i in 0..((1 << dim).max(2*dim)) {
        colors.push((rng.random(),rng.random(),rng.random()));
    }

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

    for i in 0..1 {
        world.objects.push(Object {
            body: Body {
                mass: n64(1.0),
                inertia: Inertia::Scalar { s: n64(1.0) },
                stationary: false,
                pos: Position {
                    linear: n64(3.0 * (i+1) as f64) * VecN::basis(dim, 0),
                    angular: MatN::identity(dim),
                },
                mom: Momentum {
                    linear: VecN {
                        e: (0..dim).map(|_x| n64(0.01) * n64(rng.sample(StandardNormal))).collect()
                    },
                    angular: BiVecN::basis(dim, 0, 1) + VecN {
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

        render(i, dim, &colors, &world.objects);
        println!("{}", i);

        i+=1;
    }

    let duration = start.elapsed();

    println!("{:?}", duration);
}

// ffmpeg -framerate 60 -i output_%03d.png -c:v libx264 -pix_fmt yuv420p output.mp4
fn render(n: usize, dim: usize, colors: &Vec<(f64,f64,f64)>, objects: &Vec<Object>) {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Create a new ImageBuffer with the specified dimensions and pixel type (Rgb in this case)
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let pos = n64(2.0) * VecN::basis(dim, 0) + n64(-4.0) * VecN::basis(dim, dim-1);

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
                let mut center = &object.body.pos.linear - &pos;
                center.e[0] = n64(0.0);
                center.e[1] = n64(0.0);
                center.e[dim-1] = n64(0.0);
                if center.length_sqr() < *radius*radius {
                    culled.push(object);
                }
            },
            // Collider::Polytope { maxradius, poly } => {
            //     let mut center = &object.body.pos.linear - &pos;
            //     center.e[0] = n64(0.0);
            //     center.e[1] = n64(0.0);
            //     center.e[dim-1] = n64(0.0);
            //     if center.length_sqr() < *maxradius*maxradius {
            //         culled.push(object);
            //     }
            // },
        }
    }

    // Iterate over the pixels and set their color values
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let fx = n64(2.0) * (n64(x as f64) / (IMAGE_WIDTH - 1) as f64) - n64(1.0);
        let fy = n64(-2.0) * (n64(y as f64) / (IMAGE_HEIGHT - 1) as f64) + n64(1.0);

        // Calculate RGB values based on coordinates
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        let vec = (fy * VecN::basis(dim, 0) + fx * VecN::basis(dim, 1) + VecN::basis(dim, dim-1)).normalize();
        let mut t = n64(0.0);

        for _i in 0..MAX_STEPS {
            let loc = &pos + &vec * t;

            let mut dist = N64::infinity();
            let mut id = culled.len();

            for j in 0..culled.len() {
                let d = match &culled[j].body.collider {
                    Collider::HalfSpace { normal } => {
                        (&loc - &culled[j].body.pos.linear).dot(normal)
                    },
                    Collider::Sphere { radius } => {
                        (&loc - &culled[j].body.pos.linear).length() - radius
                    },
                    // Collider::Polytope { maxradius, poly } => {
                        
                    // },
                };
                if d < dist {
                    dist = d;
                    id = j;
                }
            }

            if dist < EPS {
                let f = 10.0 / (t.raw() + 10.0);

                (r,g,b) = match &culled[id].body.collider {
                    Collider::HalfSpace { normal: _normal } => {
                        let s: i32 = loc.e.iter()
                            .map(|x| x.raw().round() as i32)
                            .sum();
                        if s % 2 == 0 {
                            (0.25,0.25,0.25)
                        } else {
                            (0.75,0.75,0.75)
                        }
                    },
                    Collider::Sphere { radius: _radius } => {
                        let c = &loc - &culled[id].body.pos.linear;
                        let mut n = 0;
                        for j in 0..dim {
                            if c.dot(&culled[id].body.pos.angular.e[j]) >= 0.0 {
                                n += 1<<j;
                            }
                        }
                        colors[n]
                    },
                    // Collider::Polytope { maxradius, poly } => {
                    //     (0.5, 0.5, 0.5)
                    // },
                };

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