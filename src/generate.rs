use crate::options::StarMapOptions;
use crate::StarMapEntry;
use rand::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;

pub fn generate_into_slice(entries: &mut [StarMapEntry], options: StarMapOptions) {
    let len = entries.len() as f32;
    let mut i: u32 = 0;
    let mut pcg_rng = Pcg64::seed_from_u64(2);

    for e in entries.iter_mut() {
        e.x = generate_x(i, i as f32 / len, options.core_size, &mut pcg_rng);
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
