use crate::options::StarMapOptions;
use crate::StarMapEntry;
use rand::prelude::*;
use rand::SeedableRng;
use std::f32::consts::PI;
use std::f32::consts::FRAC_PI_2;
use rayon::prelude::*;

const CHUNK_SIZE: usize = 64;

use rand_xoshiro::Xoshiro256Plus;

pub fn generate_into_slice(entries: &mut [StarMapEntry], options: StarMapOptions)
{
    let mut seeds: Vec<u64> = Vec::new();

    let mut rng1 = Xoshiro256Plus::seed_from_u64(options.seed);

    entries.chunks(CHUNK_SIZE).for_each(|_|{
       seeds.push(rng1.gen())
    });

    entries.par_chunks_mut(CHUNK_SIZE).zip(seeds)
        .for_each(|(c,s)|
            {
                let mut invert: bool = false;

                let mut rng = Xoshiro256Plus::seed_from_u64(s);
                for entry in c.iter_mut()
                {
                    generate_x(entry, invert, options.core_size, options.centre_distribution, rng.gen_range(0f32, 1f32));

                    generate_xy(entry, options.height, options.height_distribution, rng.gen_range(-1f32, 1f32));

                    generate_xz(entry, options.depth, options.depth_distribution, options.swirl_magnitude, rng.gen_range(-1f32, 1f32), rng.gen_range(0f32, PI));

                    invert = !invert;
                }
            });
}

fn generate_x(entry: &mut StarMapEntry, invert: bool, core_size: f32, distribution: f32, rand: f32)
{
    entry.x = skew_by_cosine_with_scale_and_modifier(rand,FRAC_PI_2, distribution) * (1f32 - core_size) + core_size;

    if invert
    {
        entry.x *= -1f32;
    }
}

fn generate_xy(entry: &mut StarMapEntry, height: f32, distribution: f32, rand: f32)
{
    rotate_around_z_0(entry, get_angle(entry.x, height, distribution, rand));
}

fn generate_xz(entry: &mut StarMapEntry, depth: f32, distribution: f32, swirl_magnitude: f32, randa: f32, randb: f32)
{
    rotate_around_y_0(entry, get_angle(entry.x, depth, distribution, randa) + swirl_magnitude * entry.x.abs() + (randb * -0.05f32));
}

fn get_angle(x: f32, max: f32, distribution: f32, rand: f32) -> f32 {
    let max_angle = get_max_angle(x, max);
    skew_by_cosine_with_scale_and_modifier(rand * max_angle, 1f32, distribution)
}

fn get_max_angle(x: f32, max_dist: f32) -> f32 {
    if x.abs() <= max_dist
    {
        return FRAC_PI_2;
    }

    let max_angle = (max_dist / x).asin();
    f32::min(max_angle, FRAC_PI_2)
}

fn skew_by_cosine_with_scale_and_modifier(n:f32, scale: f32, modifier: f32) -> f32 {
    n - n * (n * scale).cos() * modifier
}

fn rotate_around_z_0(entry: &mut StarMapEntry, angle: f32) {
    let (s, c) = angle.sin_cos();
    let x = entry.x * c - entry.y * s;
    entry.y = entry.x * s + entry.y * c;
    entry.x = x;
}

fn rotate_around_y_0(entry: &mut StarMapEntry, angle: f32) {
    let (s, c) = angle.sin_cos();
    let x = entry.x * c - entry.z * s;
    entry.z = entry.x * s + entry.z * c;
    entry.x = x;
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::StarMapEntry;

    use super::FRAC_PI_2;

    #[test]
    pub fn get_angle_when_rand_is_0p5_should_get_half_max_angle() {
        let max_angle = super::get_max_angle(1f32, 0.5f32);
        assert_eq!(super::get_angle(1f32, 0.5f32, 0f32, 0.5f32), max_angle / 2f32);
    }

    #[test]
    pub fn rotate_around_z_0_with_angle_half_pi_should_set_x_0_y_1() {
        let mut entry = StarMapEntry { x: 1f32, y: 0f32, z: 0f32, w: 0f32 };
        super::rotate_around_z_0(&mut entry, PI / 2f32);
        assert_eq!((entry.x * 10000f32).round() / 10000f32, 0f32);
        assert_eq!((entry.y * 10000f32).round() / 10000f32, 1f32);
    }

    #[test]
    pub fn rotate_around_z_0_with_angle_quarter_pi_should_set_x_0_7071_y_0_7071() {
        let mut entry = StarMapEntry { x: 1f32, y: 0f32, z: 0f32, w: 0f32 };
        super::rotate_around_z_0(&mut entry, PI / 4f32);
        assert_eq!((entry.x * 10000f32).round() / 10000f32, 0.7071f32);
        assert_eq!((entry.y * 10000f32).round() / 10000f32, 0.7071f32);
    }

    #[test]
    pub fn rotate_around_y_0_with_angle_half_pi_should_set_x_0_z_1() {
        let mut entry = StarMapEntry { x: 1f32, y: 0f32, z: 0f32, w: 0f32 };
        super::rotate_around_y_0(&mut entry, PI / 2f32);
        assert_eq!((entry.x * 10000f32).round() / 10000f32, 0f32);
        assert_eq!((entry.z * 10000f32).round() / 10000f32, 1f32);
    }

    #[test]
    pub fn rotate_around_y_0_with_angle_quarter_pi_should_set_x_0_7071_z_0_7071() {
        let mut entry = StarMapEntry { x: 1f32, y: 0f32, z: 0f32, w: 0f32 };
        super::rotate_around_y_0(&mut entry, PI / 4f32);
        assert_eq!((entry.x * 10000f32).round() / 10000f32, 0.7071f32);
        assert_eq!((entry.z * 10000f32).round() / 10000f32, 0.7071f32);
    }

    #[test]
    pub fn rotate_around_y_0_with_angle_neg_quarter_pi_should_set_x_0_7071_z_neg_0_7071() {
        let mut entry = StarMapEntry { x: 1f32, y: 0f32, z: 0f32, w: 0f32 };
        super::rotate_around_y_0(&mut entry, -PI / 4f32);
        assert_eq!((entry.x * 10000f32).round() / 10000f32, 0.7071f32);
        assert_eq!((entry.z * 10000f32).round() / 10000f32, -0.7071f32);
    }

    #[test]
    pub fn get_max_angle_when_max_dist_is_one_should_return_half_pi() {
        assert_eq!(super::get_max_angle(1f32, 1f32), FRAC_PI_2);
    }

    #[test]
    pub fn get_max_angle_when_max_dist_is_above_one_should_return_half_pi() {
        assert_eq!(super::get_max_angle(1f32, 2f32), FRAC_PI_2);
    }

    #[test]
    pub fn get_max_angle_when_max_dist_is_point_five_should_return_sixth_pi() {
        assert_eq!(super::get_max_angle(1f32, 0.5f32), PI / 6f32);
    }

    #[test]
    pub fn get_max_angle_when_dist_is_less_than_max_return_half_pi() {
        assert_eq!(super::get_max_angle(0.5f32, 1f32), FRAC_PI_2);
    }

    #[test]
    pub fn get_max_angle_when_max_dist_is_one_should_return_half_pi_neg() {
        assert_eq!(super::get_max_angle(-1f32, 1f32), FRAC_PI_2);
    }

    #[test]
    pub fn get_max_angle_when_max_dist_is_above_one_should_return_half_pi_neg() {
        assert_eq!(super::get_max_angle(-1f32, 2f32), FRAC_PI_2);
    }

    #[test]
    pub fn get_max_angle_when_max_dist_is_point_five_should_return_sixth_pi_neg() {
        assert_eq!(super::get_max_angle(-1f32, 0.5f32), -PI / 6f32);
    }

    #[test]
    pub fn get_max_angle_when_dist_is_less_than_max_return_half_pi_neg() {
        assert_eq!(super::get_max_angle(-0.5f32, 1f32), FRAC_PI_2);
    }
}