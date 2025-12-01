// pub mod polytope;
// use polytope::Polytope as Polytope;

use macroquad::prelude::*;
use nalgebra::{SMatrix, SVector, Vector2};
use ::rand::{prelude::*, rng};
use rand_distr::StandardNormal;

pub(crate) trait ScalarType: nalgebra::Scalar + num_traits::identities::Zero + std::ops::AddAssign + std::ops::MulAssign + std::ops::Mul + nalgebra::ClosedMulAssign + std::cmp::PartialOrd {}
impl<T> ScalarType for T where T: nalgebra::Scalar + num_traits::identities::Zero + std::ops::AddAssign + std::ops::MulAssign + std::ops::Mul + nalgebra::ClosedMulAssign + std::cmp::PartialOrd {}

const DIMENSION: usize = 4;
const BIDIMENSION: usize = (DIMENSION*DIMENSION - DIMENSION) / 2;
type Scalar = f64;
type Vector = SVector<Scalar, DIMENSION>;
type BiVector = SVector<Scalar, BIDIMENSION>;
type Matrix = SMatrix<Scalar, DIMENSION, DIMENSION>;

type Vec2 = Vector2<f32>;

static FRAMERATE: i16 = 360;

// static verts: Vec<Vector> = ncubevertices<Scalar, DIMENSION>();
static verts: [Vector; 16] = [
    Vector::new(-1.0,-1.0,-1.0,-1.0),
    Vector::new(-1.0,-1.0,-1.0, 1.0),
    Vector::new(-1.0,-1.0, 1.0,-1.0),
    Vector::new(-1.0,-1.0, 1.0, 1.0),
    Vector::new(-1.0, 1.0,-1.0,-1.0),
    Vector::new(-1.0, 1.0,-1.0, 1.0),
    Vector::new(-1.0, 1.0, 1.0,-1.0),
    Vector::new(-1.0, 1.0, 1.0, 1.0),
    Vector::new( 1.0,-1.0,-1.0,-1.0),
    Vector::new( 1.0,-1.0,-1.0, 1.0),
    Vector::new( 1.0,-1.0, 1.0,-1.0),
    Vector::new( 1.0,-1.0, 1.0, 1.0),
    Vector::new( 1.0, 1.0,-1.0,-1.0),
    Vector::new( 1.0, 1.0,-1.0, 1.0),
    Vector::new( 1.0, 1.0, 1.0,-1.0),
    Vector::new( 1.0, 1.0, 1.0, 1.0),
];

static edges: [(usize,usize); 32] = [
    (0b0000,0b0001),
    (0b0000,0b0010),
    (0b0000,0b0100),
    (0b0000,0b1000),
    (0b0001,0b0011),
    (0b0001,0b0101),
    (0b0001,0b1001),
    (0b0010,0b0011),
    (0b0010,0b0110),
    (0b0010,0b1010),
    (0b0011,0b0111),
    (0b0011,0b1011),
    (0b0100,0b0101),
    (0b0100,0b0110),
    (0b0100,0b1100),
    (0b0101,0b0111),
    (0b0101,0b1101),
    (0b0110,0b0111),
    (0b0110,0b1110),
    (0b0111,0b1111),
    (0b1000,0b1001),
    (0b1000,0b1010),
    (0b1000,0b1100),
    (0b1001,0b1011),
    (0b1001,0b1101),
    (0b1010,0b1011),
    (0b1010,0b1110),
    (0b1011,0b1111),
    (0b1100,0b1101),
    (0b1100,0b1110),
    (0b1101,0b1111),
    (0b1110,0b1111),
];

#[macroquad::main("Arcade4D")]
async fn main() {
    let mut rng = rng();

    let mut camera: (Vector, Matrix) = (
        Vector::new(0.0,0.0,-4.0,0.0),
        Matrix::new(
            1.0,0.0,0.0,0.0,
            0.0,1.0,0.0,0.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0,0.0,1.0,
        ),
    );

    let mut randrot: Matrix = Matrix::new(
        0.0,rng.sample(StandardNormal),rng.sample(StandardNormal),rng.sample(StandardNormal),
        0.0,0.0,rng.sample(StandardNormal),rng.sample(StandardNormal),
        0.0,0.0,0.0,rng.sample(StandardNormal),
        0.0,0.0,0.0,0.0,
    );
    randrot = (randrot - randrot.transpose()) / 100.0;
    randrot = randrot.exp();

    let minimum_frame_time = 1.0 / (FRAMERATE as f64);
    let mut time = get_time();
    let mut dta: f32 = minimum_frame_time as f32;
    let mut frame_time: f64 = 0.0;

    loop {
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.0;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
        let new_time = get_time();
        let dt = new_time - time;
        time = new_time;
        dta = dta * 0.99 + (dt as f32) * 0.01;

        let scale: f32 = (screen_width().min(screen_height())) / 2.0;
        let off: Vec2 = Vec2::new(screen_width(), screen_height()) / 2.0;



        if is_key_down(KeyCode::W) {
            camera.0 += camera.1.column(2) / 10.0;
        }
        if is_key_down(KeyCode::S) {
            camera.0 -= camera.1.column(2) / 10.0;
        }

        if is_key_down(KeyCode::A) {
            camera.0 -= camera.1.column(0) / 10.0;
        }
        if is_key_down(KeyCode::D) {
            camera.0 += camera.1.column(0) / 10.0;
        }

        if is_key_down(KeyCode::Q) {
            camera.0 -= camera.1.column(3) / 10.0;
        }
        if is_key_down(KeyCode::E) {
            camera.0 += camera.1.column(3) / 10.0;
        }

        clear_background(BLACK);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(
            format!("fps: {}", 1.0 / dta).as_str(),
            screen_width() / 2.0,
            20.0,
            20.0,
            DARKGRAY,
        );

        let mut projectedverts: Vec<Vec2> = Vec::new();

        for i in 0..verts.len() {
            projectedverts.push(off + scale * project(worldtocamera(verts[i], camera), 0, 1, 2));
        }

        for i in 0..edges.len() {
            let p1: Vec2 = projectedverts[edges[i].0];
            let p2: Vec2 = projectedverts[edges[i].1];
            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);
        }

        camera.0 = randrot * camera.0;
        camera.1 *= randrot;



        frame_time = get_time() - time;
        next_frame().await;
    }
}

fn project(pos: Vector, d1: usize, d2: usize, dp: usize) -> Vec2 {
    return Vec2::new((pos[d1] / pos[dp]) as f32, (pos[d2] / pos[dp]) as f32);
}

fn worldtocamera(pos: Vector, camera: (Vector, Matrix)) -> Vector {
    return camera.1.transpose() * (pos - camera.0);
}

fn cameratoworld(pos: Vector, camera: (&Vector, &Matrix)) -> Vector {
    return (camera.1 * pos) + camera.0;
}