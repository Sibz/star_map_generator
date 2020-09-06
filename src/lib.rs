pub mod options;
mod star_map_c;
#[cfg(test)]
mod tests;

pub use options::StarMapOptions;
use star_map_c::StarMapVecC;

pub fn generate_star_map_in_preallocated_memory(
    options: StarMapOptions,
    pointer: *mut StarMapEntry,
) -> Result<(), String> {
    let result = options.validate();

    if result.is_err() {
        return result;
    }

    let mut x = StarMapVecC {
        ptr: pointer,
        len: options.object_count,
    };

    generate_into_slice(&mut x);

    Ok(())
}

pub fn generate(options: StarMapOptions) -> Vec<StarMapEntry> {
    let mut x: Vec<StarMapEntry> = vec![
        StarMapEntry {
            x: 0f32,
            y: 0f32,
            z: 0f32,
            w: 0f32,
        };
        options.object_count as usize
    ];
    generate_into_slice(&mut x);
    x
}

fn generate_into_slice(entries: &mut [StarMapEntry]) {
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

#[repr(C)]
#[derive(Clone)]
pub struct StarMapEntry {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// impl Drop for CVec {
//     fn drop(&mut self) {
//         unsafe { deallocate_data(self.ptr) };
//     }
// }
