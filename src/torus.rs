#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! [package]
//! name = "donut"
//! version = "0.1.0"

// from https://www.a1k0n.net/2021/01/13/optimizing-donut.html

const THETA_SPACING: f32 = 0.07;
const PHI_SPACING: f32 = 0.02;
const SCREEN_SIZE: usize = 1760; // original was 1760

fn rotation(t: f32, mut x: f32, mut y: f32) -> (f32, f32) {
    let mut f = x;
    x -= t * y;
    y += t * f;
    f = (3. - (x * x) - (y * y)) / 2.;
    (x * f, y * f)
}

fn main() {
    let mut a: f32 = 0.;
    let mut e: f32 = 1.;
    let mut c: f32 = 1.;
    let mut d: f32 = 0.;
    loop {
        let mut z: [f32; SCREEN_SIZE] = [0.; SCREEN_SIZE];
        let (mut g, mut h): (f32, f32) = (0., 1.);
        let mut b: [char; SCREEN_SIZE] = [' '; SCREEN_SIZE];
        for _ in 0..90 {
            let (mut G, mut H): (f32, f32) = (0., 1.);
            for _ in 0..314 {
                let A: f32 = h + 2.;
                let D: f32 = 1. / ((G * A * a + g * e) + 5.);
                let t: f32 = (G * A * e) - (g * a);
                let x: i32 = (40. + (30. * D * ((H * A * d) - (t * c)))).round() as i32;
                let y: i32 = (12. + (15. * D * ((H * A * c) + (t * d)))).round() as i32;
                let o: i32 = x + (80 * y);
                let N: i32 = (8. * ((((g * a) - (G * h * e)) * d) - (G * h * a) - (g * e) - (H * h * c)))
                    as i32;
                if 0 < y && y < 22 && 0 < x && x < 80 && D > z[o as usize] {
                    z[o as usize] = D;
                    b[o as usize] = char::from(b".,-~:;=!*#$@"[if N > 0 { N as usize } else { 0 }]);
                }
                (H, G) = rotation(PHI_SPACING, H, G);
            }
            (h, g) = rotation(THETA_SPACING, h, g);
        }
        for k in 0..=SCREEN_SIZE {
            print!("{}", if k % 80 != 0 { b[k] } else { '\n' });
        }
        (e, a) = rotation(0.04, e, a);
        (d, c) = rotation(0.02, d, c);
        std::thread::sleep(std::time::Duration::from_millis(20));

        print!("\x1b[23A");
    }
}
