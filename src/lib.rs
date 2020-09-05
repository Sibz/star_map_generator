#[cfg(test)]
mod tests;

use std::slice;

pub mod options;
pub use options::StarMapOptions;

pub fn generate_star_map_in_preallocated_memory(
    options: StarMapOptions,
    pointer: *mut StarMapEntry,
) -> Result<(), String> {
    let mut x = StarMapVecC {
        ptr: pointer,
        len: options.object_count,
    };
    generate_into_slice(&mut x);
    Ok(())
    //Err(String::from("re"))
}

pub fn generate(options: StarMapOptions) -> Vec<StarMapEntry> {
    let mut x: Vec<StarMapEntry> = vec![
        StarMapEntry {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        };
        options.object_count as usize
    ];
    generate_into_slice(&mut x);
    x
}

fn generate_into_slice(entries: &mut [StarMapEntry]) {
    entries.len();
    let mut i: u32 = 0;
    for e in entries.iter_mut() {
        e.x = i;
        e.y = i + 1;
        e.z = i + 2;
        e.w = i + 3;
        i = i + 1;
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct StarMapEntry {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

struct StarMapVecC {
    ptr: *mut StarMapEntry,
    len: u32,
}

impl std::ops::Deref for StarMapVecC {
    type Target = [StarMapEntry];

    fn deref(&self) -> &[StarMapEntry] {
        unsafe { slice::from_raw_parts(self.ptr, self.len as usize) }
    }
}

impl std::ops::DerefMut for StarMapVecC {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len as usize) }
    }
}

// impl Drop for CVec {
//     fn drop(&mut self) {
//         unsafe { deallocate_data(self.ptr) };
//     }
// }
