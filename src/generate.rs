use crate::options::StarMapOptions;
use crate::StarMapEntry;
use rand::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

pub fn generate_into_slice(entries: &mut [StarMapEntry], options: StarMapOptions)
{
    let mut invert: bool = false;
    let mut pcg_rng = Pcg64::seed_from_u64(options.seed);

    for e in entries.iter_mut() {
        let x = generate_x(
            invert,
            options.core_size,
            options.centre_distribution,
            &mut pcg_rng,
        );
        let (x1, y1) = random_rotate2d_with_max_distance_and_distribute(
            (x, e.y),
            options.height,
            options.height_distribution,
            &mut pcg_rng,
        );

        let xz = random_rotate2d_with_max_distance_and_distribute(
            (x1, e.z),
            1f32,
            options.depth_distribution,
            &mut pcg_rng,
        );

        let (x3, z2) = apply_swirl(xz, options.swirl_magnitude, &mut pcg_rng);

        e.x = x3;
        e.y = y1;
        e.z = z2;

        invert = !invert;
    }
}

fn generate_x(invert: bool, core_size: f32, distribution: f32, rng: &mut Pcg64) -> f32
{
    let rnd = rng.gen_range(0f32, 1f32 + std::f32::EPSILON);
    let rnd_scaled = rnd * FRAC_PI_2;
    let sine = rnd_scaled.cos();
    let sine_warp = sine * distribution;
    let rnd = rnd - rnd * sine_warp;
    let rnd = rnd * (1f32 - core_size) + core_size;

    if invert {
        rnd
    } else {
        -rnd
    }
}

fn random_rotate2d_with_max_distance_and_distribute(
    xy: (f32, f32),
    max: f32,
    distribution: f32,
    rng: &mut Pcg64,
) -> (f32, f32)
{
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
    let sine = (angle_scaled).sin();

    // a flatter or taller wave
    let sine_distribution = sine * distribution; // * warp;

    let angle = (angle - max_angle) - (angle - max_angle) * sine_distribution;

    rotate_2d(angle, xy)
}

fn apply_swirl(xz: (f32, f32), swirl_magnitude: f32, rng: &mut Pcg64) -> (f32, f32)
{
    let (x, z) = xz;
    let radian_rotation = swirl_magnitude * PI * 2f32 * ((x.abs() + z.abs()) / 2f32);
    let rand_rotation = rng.gen_range(radian_rotation * 0.8, radian_rotation);
    rotate_2d(rand_rotation, xz)
}

fn rotate_2d(angle: f32, xy: (f32, f32)) -> (f32, f32)
{
    let (x, y) = xy;
    let (s, c) = angle.sin_cos();
    (x * c - y * s, x * s + y * c)
}
