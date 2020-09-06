use crate::options::StarMapOptions;
use crate::StarMapEntry;
use interpolation::*;
use rand::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;

pub fn generate_into_slice(entries: &mut [StarMapEntry], options: StarMapOptions) {
    let len = entries.len() as f32;
    let mut i: u32 = 0;
    let mut pcg_rng = Pcg64::seed_from_u64(options.seed);

    for e in entries.iter_mut() {
        let x = generate_x(i, i as f32 / len, options.core_size, &mut pcg_rng);
        let (x1, y1) = generate_xy(x, options.height, &mut pcg_rng);

        e.x = x1;
        e.y = y1;
        // e.y = i + 1f32;
        // e.z = i + 2f32;
        // e.w = i + 3f32;
        i = i + 1;
    }
}

fn generate_x(i: u32, percent_position: f32, core_size: f32, rng: &mut Pcg64) -> f32 {
    if i % 2 == 0 {
        rng.gen_range(
            0f32 + core_size,
            f32::max(
                core_size + std::f32::EPSILON,
                1f32 + std::f32::EPSILON - percent_position,
            ),
        )
    } else {
        -rng.gen_range(
            0f32 + core_size,
            f32::max(
                core_size + std::f32::EPSILON,
                1f32 + std::f32::EPSILON - percent_position,
            ),
        )
    }
}

fn generate_xy(x: f32, height: f32, rng: &mut Pcg64) -> (f32, f32) {
    let angle = rng.gen_range(0f32, std::f32::consts::FRAC_PI_2);

    let angle = lerp(&-angle, &angle, &(rng.gen_range(0f32, 1f32)));
    rotate_2d(angle, x, 0f32)
}

fn rotate_2d(angle: f32, x: f32, y: f32) -> (f32, f32) {
    let s_c = angle.sin_cos();
    (x * s_c.1 - y * s_c.0, x * s_c.0 + y * s_c.1)
}
