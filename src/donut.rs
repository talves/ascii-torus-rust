#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! [package]
//! name = "donut"
//! version = "0.1.0"
//! edition = "2021"

const THETA_SPACING: f32 = 0.070;
const PHI_SPACING: f32 = 0.020;
const R1: i32 = 1;
const R2: i32 = 2;
const K2: i32 = 5;
const Z_DIST: i32 = 3;
const SCREEN_WIDTH: i32 = 120;
const SCREEN_HEIGHT: i32 = 22;
const K1: i32 = SCREEN_WIDTH*(K2 - Z_DIST)*3/(8*(R1+R2));
const BUFFER_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const SCALE_SIZE: usize = BUFFER_SIZE * 4;

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    loop {
        let mut loc_z: [f32; SCALE_SIZE] = [0.; SCALE_SIZE];
        let mut tan_char: [char; BUFFER_SIZE] = [' '; BUFFER_SIZE];
        for _theta in (0..(std::f64::consts::PI * 200.0) as i32).step_by((THETA_SPACING * 100.0) as usize) {
            let theta = _theta as f32 * 0.01;
            let mut h: f32;
            let mut z: f32;
            let mut ooz: f32;
            let mut t: f32;
            let mut x: i32;
            let mut y: i32;
            let mut o: usize;
            let mut tan_dist: usize;
            for _phi in (0..(std::f64::consts::PI * 200.0) as i32).step_by((PHI_SPACING * 100.0) as usize) {
                let phi = _phi as f32 * 0.01;
                h = (R1 as f32 * theta.cos() + R2 as f32) as f32;
                z = phi.sin() * h * a.sin() + theta.sin() * a.cos() + K2 as f32;
                ooz = 1.0 / z;
                t = phi.sin() * h * a.cos() - theta.sin() * a.sin();
                x = ((SCREEN_WIDTH / 2) as f32 + K1 as f32 * ooz * (phi.cos() * h * b.cos() - t * b.sin())) as i32;
                y = ((SCREEN_HEIGHT / 2) as f32 + 2.0 + (K1/2) as f32 * ooz * (phi.cos() * h * b.sin() + t * b.cos())) as i32;
                o = (x + SCREEN_WIDTH * y) as usize;
                tan_dist = (8.0 * ((theta.sin() * a.sin() - phi.sin() * theta.cos() * a.cos()) * b.cos() - phi.sin() * theta.cos() * a.sin() - theta.sin() * a.cos() - phi.cos() * theta.cos() * b.sin())) as usize;
                if SCREEN_HEIGHT > y && y > 0 && x > 0 && SCREEN_WIDTH > x && ooz > loc_z[o] {
                    loc_z[o] = ooz;
                    tan_char[o] = b".,-~:;=!*#$@"[if tan_dist > 0 { tan_dist } else { 0 }] as char
                }
            }
        }
        print!("\x1b[H");
        for k in 0..=BUFFER_SIZE {
            print!(
                "{}",
                if k % SCREEN_WIDTH as usize != 0 {
                    tan_char[k]
                } else {
                    '\n' as char
                }
            )
        }
        a += 0.040;
        b += 0.020;
        std::thread::sleep(std::time::Duration::from_millis(15));
        print!(" donut.rs! \x1b[23A");
    }
}
