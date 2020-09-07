use crate::options::StarMapOptions;
use crate::StarMapEntry;
use interpolation::*;
use rand::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

pub fn generate_into_slice(entries: &mut [StarMapEntry], options: StarMapOptions) {
    let len = entries.len() as f32;
    let mut i: u32 = 0;
    let mut pcg_rng = Pcg64::seed_from_u64(options.seed);

    const WARP: f32 = 0.1;

    for e in entries.iter_mut() {
        let x = generate_x(i, i as f32 / len, options.core_size, &mut pcg_rng);
        //e.x = x;
        let (x1, y1) = random_rotate2d_with_max_distance_and_warp(
            (x, e.y),
            options.height,
            WARP,
            &mut pcg_rng,
        );

        e.x = x1;
        e.y = y1;
        // e.y = i + 1f32;
        // e.z = i + 2f32;
        // e.w = i + 3f32;
        i = i + 1;
    }
}

fn generate_x(i: u32, percent_position: f32, core_size: f32, rng: &mut Pcg64) -> f32 {
    let rnd = rng.gen_range(0f32, 1f32 + std::f32::EPSILON);
    let rnd_scaled = rnd * FRAC_PI_2;
    let sine = rnd_scaled.cos();
    let sine_warp = sine * 0.575f32;

    let rnd = rnd - rnd * sine_warp;

    let rnd = rnd * (1f32 - core_size) + core_size;

    if i % 2 == 0 {
        rnd
    } else {
        -rnd
    }
}

fn random_rotate2d_with_max_distance_and_warp(
    xy: (f32, f32),
    max: f32,
    warp: f32,
    rng: &mut Pcg64,
) -> (f32, f32) {
    let mut max_angle = (max / xy.0).asin().abs();
    if max_angle.is_nan() {
        max_angle = FRAC_PI_2;
    } else {
        max_angle = f32::min(max_angle, FRAC_PI_2);
    }

    //  zero to max angle
    //let angle = lerp(&0f32, &(max_angle * 2f32), &(rng.gen_range(0f32, 1f32)));
    let angle = rng.gen_range(0f32, (max_angle * 2f32) + f32::EPSILON);

    // will be somewhere in between zero to PI
    let angle_scaled = angle / (max_angle * 2f32) * PI;

    // will be somewhere in half sine wave 0 to 1 to 0
    let inverse_sine = (angle_scaled).sin();

    // a flatter or taller wave
    let inverse_sine_warp = inverse_sine; // * warp;

    let angle = (angle - max_angle) - (angle - max_angle) * inverse_sine_warp * 0.75f32;

    rotate_2d(angle, xy)
}

fn rotate_2d(angle: f32, xy: (f32, f32)) -> (f32, f32) {
    let (x, y) = xy;
    let (s, c) = angle.sin_cos();
    (x * c - y * s, x * s + y * c)
}
