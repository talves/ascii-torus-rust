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
    let (mut cos_a, mut sin_a): (f32, f32) = (0., 1.);
    let (mut cos_b, mut sin_b): (f32, f32) = (1., 0.);
    loop {
        let mut loc_z: [f32; SCALE_SIZE] = [0.; SCALE_SIZE];
        let mut tan_char: [char; SCREEN_SIZE] = [' '; SCREEN_SIZE];
        let (mut cos_theta, mut sin_theta): (f32, f32) = (0., 1.0);
        for _theta in 0..90 {
            let (mut cos_phi, mut sin_phi): (f32, f32) = (0., 1.0);
            let mut circle_x: f32;
            let mut z: f32;
            let mut ooz: f32;
            let mut t: f32;
            let mut x: i32;
            let mut y: i32;
            let mut o: usize;
            let mut tan_dist: usize;
            for _phi in 0..314 {
                circle_x = sin_theta + 2.;
                z = cos_phi * circle_x * cos_a + cos_theta * sin_a + 5.;
                ooz = 1. / z;
                t = (cos_phi * circle_x * sin_a) - (cos_theta * cos_a);
                x = ((CHAR_WIDTH / 2) as f32
                    + (VIEW_X_POS * ooz * (sin_phi * circle_x * sin_b - t * cos_b)))
                    as i32;
                y = (11.0 + (VIEW_Y_POS * ooz * (sin_phi * circle_x * cos_b + t * sin_b))) as i32;
                o = (x + (CHAR_WIDTH * y)) as usize;
                tan_dist = (8.
                    * (((cos_theta * cos_a - cos_phi * sin_theta * sin_a) * sin_b)
                        - cos_phi * sin_theta * cos_a
                        - cos_theta * sin_a
                        - sin_phi * sin_theta * cos_b)) as usize;
                if 0 < y && y < 22 && 0 < x && x < CHAR_WIDTH && ooz > loc_z[o] {
                    loc_z[o] = ooz;
                    tan_char[o] = b".,-~:;=!*#$@"[if tan_dist > 0 { tan_dist } else { 0 }] as char
                }
                (sin_phi, cos_phi) = rotation(0.02, sin_phi, cos_phi);
            }
            (sin_theta, cos_theta) = rotation(0.07, sin_theta, cos_theta);
        }
        for k in 0..=SCREEN_SIZE {
            print!(
                "{}",
                if k % CHAR_WIDTH as usize != 0 {
                    tan_char[k]
                } else {
                    '\n' as char
                }
            )
        }
        (sin_a, cos_a) = rotation(THETA_SPACING, sin_a, cos_a);
        (sin_b, cos_b) = rotation(PHI_SPACING, sin_b, cos_b);
        std::thread::sleep(std::time::Duration::from_millis(15));

        print!(" donut.rs! \x1b[23A");
    }
}
