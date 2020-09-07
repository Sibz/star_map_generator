use crate::options::StarMapOptions;
use crate::StarMapEntry;
use interpolation::*;
use rand::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::f32::consts::FRAC_PI_2;

pub fn generate_into_slice(entries: &mut [StarMapEntry], options: StarMapOptions) {
    let len = entries.len() as f32;
    let mut i: u32 = 0;
    let mut pcg_rng = Pcg64::seed_from_u64(options.seed);

    const WARP: f32 = 0.1;

    for e in entries.iter_mut() {
        let x = generate_x(i, i as f32 / len, options.core_size, &mut pcg_rng);
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
    let min = 0f32 + core_size;
    let true_max = 1f32 + std::f32::EPSILON - percent_position + min * percent_position;
    // let max = f32::max(
    //     core_size * 2f32,
    //     ,
    // );
    let rnd = rng.gen_range(min, true_max);
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
    let mut max_angle = (max/* * ((1f32 - (max.sin())) * warp)*/ / xy.0).asin();
    if max_angle.is_nan() {
        max_angle = FRAC_PI_2;
    } else {
        max_angle = f32::min(max_angle, FRAC_PI_2);
    }

    let angle = lerp(&-max_angle, &max_angle, &(rng.gen_range(0f32, 1f32)));

    rotate_2d(angle, xy)
}

fn rotate_2d(angle: f32, xy: (f32, f32)) -> (f32, f32) {
    let (x, y) = xy;
    let (s, c) = angle.sin_cos();
    (x * c - y * s, x * s + y * c)
}
