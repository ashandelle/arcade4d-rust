// pub mod polytope;
// use polytope::Polytope as Polytope;

use macroquad::prelude::*;

static FRAMERATE: i16 = 360;

#[macroquad::main("Arcade4D")]
async fn main() {
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



        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(
            format!("fps: {}", 1.0 / dta).as_str(),
            screen_width() / 2.0,
            20.0,
            20.0,
            DARKGRAY,
        );



        frame_time = get_time() - time;

        next_frame().await;
    }
}
