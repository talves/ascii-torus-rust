#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! [package]
//! name = "donut"
//! version = "0.1.0"
//! edition = "2021"

// from https://www.a1k0n.net/2021/01/13/optimizing-donut.html

const THETA_SPACING: f32 = 0.07;
const PHI_SPACING: f32 = 0.02;
const VIEW_X_POS: f32 = 30.0; // larger zooms in
const VIEW_Y_POS: f32 = VIEW_X_POS / 2.0;
const CHAR_WIDTH: i32 = 120; // original is 80
const SCREEN_SIZE: usize = (22 * CHAR_WIDTH) as usize; // original is 1760
const SCALE_SIZE: usize = SCREEN_SIZE * 4; // original is 7040

fn rotation(tan0: f32, mut x: f32, mut y: f32) -> (f32, f32) {
    let orig_x = x.clone();
    x -= tan0 * y;
    y += tan0 * orig_x;
    // Magnitude adjustment
    let f = (3. - (x * x) - (y * y)) / 2.;
    // Moves x and y back to the unit circle
    (x * f, y * f)
}

fn main() {
    let mut sin_a: f32 = 0.;
    let mut cos_a: f32 = 1.;
    let mut sin_b: f32 = 1.;
    let mut cos_b: f32 = 0.;
    loop {
        let mut z: [f32; SCALE_SIZE] = [0.; SCALE_SIZE]; // z buffer
        let mut b: [char; SCREEN_SIZE] = [' '; SCREEN_SIZE]; // char buffer
        let (mut cos_theta, mut sin_theta): (f32, f32) = (1., 0.);
        for _i in 0..90 {
            let (mut cos_phi, mut sin_phi): (f32, f32) = (1., 0.);
            for _j in 0..314 {
                // the x,y coordinate of the circle, before revolving
                let circle_x: f32 = cos_theta + 2.;
                let circle_y: f32 = sin_theta * 1.;
                // One over z = (sin_phi * circle_x * sin_a + circle_y * cos_a + 5.)
                let ooz: f32 = 1. / (sin_phi * circle_x * sin_a + circle_y * cos_a + 5.);
                let t: f32 = (sin_phi * circle_x * cos_a) - (circle_y * sin_a);
                let x: i32 = ((CHAR_WIDTH / 2) as f32 + (VIEW_X_POS * ooz * ((cos_phi * circle_x * cos_b) - (t * sin_b)))).round() as i32;
                let y: i32 = (11. + (VIEW_Y_POS * ooz * ((cos_phi * circle_x * sin_b) + (t * cos_b)))).round() as i32;
                let o: usize = (x + (CHAR_WIDTH * y)) as usize;
                let tan_dist: usize = (8.
                    * ((((sin_theta * sin_a) - (sin_phi * cos_theta * cos_a)) * cos_b)
                        - (sin_phi * cos_theta * sin_a)
                        - (sin_theta * cos_a)
                        - (cos_phi * cos_theta * sin_b))) as usize;
                if 0 < y && y < 22 && 0 < x && x < CHAR_WIDTH && ooz > z[o] {
                    z[o] = ooz;
                    b[o] = char::from(b".,-~:;=!*#$@"[if tan_dist > 0 { tan_dist } else { 0 }]);
                }
                (cos_phi, sin_phi) = rotation(PHI_SPACING, cos_phi, sin_phi);
            }
            (cos_theta, sin_theta) = rotation(THETA_SPACING, cos_theta, sin_theta);
        }
        for k in 0..=SCREEN_SIZE {
            print!("{}", if k % CHAR_WIDTH as usize != 0 { b[k] } else { '\n' });
        }
        (cos_a, sin_a) = rotation(0.04, cos_a, sin_a);
        (cos_b, sin_b) = rotation(0.02, cos_b, sin_b);
        std::thread::sleep(std::time::Duration::from_millis(15));

        print!(" donut.rs! \x1b[23A");
    }
}
