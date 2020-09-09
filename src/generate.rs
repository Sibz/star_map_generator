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

    let mut rng = Pcg64::seed_from_u64(options.seed);

    for e in entries.iter_mut()
    {
        let a = rng.gen_range(0f32,1f32);
        let b = rng.gen_range(0f32,1f32);
        let c = rng.gen_range(0f32,1f32);
        let d = rng.gen_range(0f32,1f32);

        //test(e);

        generate_x(invert, options.core_size, options.centre_distribution, e, a);

        let (x1, y1) =
            random_rotate2d_with_max_distance_and_distribute(
                (e.x, e.y),
                options.height,
                options.height_distribution,
                b,
            );

        let xz =
            random_rotate2d_with_max_distance_and_distribute(
                (x1, e.z),
                1f32,
                options.depth_distribution,
                c,
            );

        let (x3, z2) = apply_swirl(xz, options.swirl_magnitude, d);

        e.x = x3;
        e.y = y1;
        e.z = z2;

        invert = !invert;
    }
}

// fn test(abc: &mut StarMapEntry)
// {
//     abc.x = 1f32;
// }

#[inline(always)]
fn generate_x(invert: bool, core_size: f32, distribution: f32, e: &mut StarMapEntry, a: f32)
{
    e.x = scale(a, 1f32, FRAC_PI_2);
    e.x = skew_by_cosine_with_distribution(e.x, distribution);
    e.x = shift(e.x, core_size);
    //let rnd = rnd * (1f32 - core_size) + core_size;

    if invert {
        e.x = -e.x;
    }
}

#[inline(always)]
fn skew_by_cosine_with_distribution(n: f32, distribution: f32) -> f32
{
    n - n * n.cos() * distribution
}

#[inline(always)]
fn scale(n: f32, original_max: f32, new_max: f32) -> f32
{
    n / original_max * new_max
}

#[inline(always)]
fn shift(n: f32, val: f32) -> f32
{
    n * (1f32 - val) + val
}

#[inline(always)]
fn random_rotate2d_with_max_distance_and_distribute(
    xy: (f32, f32),
    max: f32,
    distribution: f32,
    a: f32,
) -> (f32, f32)
{
    let mut max_angle = (max / xy.0).asin().abs() * 2f32;
    if max_angle.is_nan() {
        max_angle = PI;
    } else {
        max_angle = f32::min(max_angle, PI);
    }

    let angle = skew_by_cosine_with_distribution(
        scale(a * max_angle, 1f32, PI) - max_angle / 2f32,
        distribution,
    );

    rotate_2d(angle, xy)
}
#[inline(always)]
fn apply_swirl(xz: (f32, f32), swirl_magnitude: f32, rng: f32) -> (f32, f32)
{
    let (x, z) = xz;
    let radian_rotation = swirl_magnitude * PI * (x.abs() + z.abs());
    let rand_rotation = radian_rotation + (rng * -0.05f32) * PI;
    rotate_2d(rand_rotation, xz)
}

#[inline(always)]
fn rotate_2d(angle: f32, xy: (f32, f32)) -> (f32, f32)
{
    let (x, y) = xy;
    let (s, c) = angle.sin_cos();
    (x * c - y * s, x * s + y * c)
}
