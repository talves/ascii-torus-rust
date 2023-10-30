#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! [package]
//! name = "donut"
//! version = "0.1.0"
//! edition = "2021"

const THETA_SPACING: f32 = 0.070;
const PHI_SPACING: f32 = 0.020;

fn rotation(t: f32, mut x: f32, mut y: f32) -> (f32, f32) {
    let orig_x = x.clone();
    x -= t * y;
    y += t * orig_x;
    let f: f32 = (3. - x * x - y * y) / 2.;
    (x * f, y * f)
}
fn main() {
    let (mut cos_a, mut sin_a): (f32, f32) = (0., 1.);
    let (mut cos_b, mut sin_b): (f32, f32) = (1., 0.);
    loop {
        let mut loc_z: [f32; 1760] = [0.; 1760];
        let mut tan_char: [char; 1760] = [' '; 1760];
        let (mut cos_theta, mut sin_theta): (f32, f32) = (0., 1.);
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
                circle_x = sin_theta + 2.0;
                z = cos_phi * circle_x * cos_a + cos_theta * sin_a + 5.;
                ooz = 1. / z;
                t = (cos_phi * circle_x * sin_a) - (cos_theta * cos_a);
                x = (40.0 + (30.0 * ooz * (sin_phi * circle_x * sin_b - t * cos_b))) as i32;
                y = (12.0 + (15. * ooz * (sin_phi * circle_x * cos_b + t * sin_b))) as i32;
                o = (x + (80 * y)) as usize;
                tan_dist = (8.
                    * (((cos_theta * cos_a - cos_phi * sin_theta * sin_a) * sin_b)
                        - cos_phi * sin_theta * cos_a
                        - cos_theta * sin_a
                        - sin_phi * sin_theta * cos_b)) as usize;
                if 0 < y && y < 22 && 0 < x && x < 80 && ooz > loc_z[o] {
                    loc_z[o] = ooz;
                    tan_char[o] = b".,-~:;=!*#$@"[if tan_dist > 0 { tan_dist } else { 0 }] as char
                }
                (sin_phi, cos_phi) = rotation(0.02, sin_phi, cos_phi);
            }
            (sin_theta, cos_theta) = rotation(0.07, sin_theta, cos_theta);
        }
        for k in 0..=1760 {
            print!(
                "{}",
                if k % 80 != 0 {
                    tan_char[k]
                } else {
                    '\n' as char
                }
            )
        }
        (sin_a, cos_a) = rotation(THETA_SPACING, sin_a, cos_a);
        (sin_b, cos_b) = rotation(PHI_SPACING, sin_b, cos_b);
        std::thread::sleep(std::time::Duration::from_millis(15));
        print!("\x1b[23A");
    }
}
