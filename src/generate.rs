use crate::options::StarMapOptions;
use crate::StarMapEntry;

pub fn generate_into_slice(entries: &mut [StarMapEntry], options: StarMapOptions) {
    entries.len();
    let mut i: f32 = 0f32;
    for e in entries.iter_mut() {
        e.x = i;
        e.y = i + 1f32;
        e.z = i + 2f32;
        e.w = i + 3f32;
        i = i + 1f32;
    }
}

fn generate_x(index: u32) -> f32 {
    0.1
}
