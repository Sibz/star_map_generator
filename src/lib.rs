#![feature(test)]

mod generate;
pub mod options;
mod star_map_c;
#[cfg(test)]
mod tests;

pub use options::StarMapOptions;
use star_map_c::StarMapVecC;

pub fn generate_star_map_in_preallocated_memory(
    options: StarMapOptions,
    pointer: *mut StarMapEntry,
) -> Result<(), String>

{
    let result = options.validate();

    if result.is_err() {
        return result;
    }

    let mut x = StarMapVecC {
        ptr: pointer,
        len: options.object_count,
    };

    generate::generate_into_slice(&mut x, options);

    Ok(())
}


pub fn generate_star_map_in_preallocated_memory_with_chunk_size(
    options: StarMapOptions,
    pointer: *mut StarMapEntry,
    chunk_size: usize) -> Result<(), String>
{
    let result = options.validate();

    if result.is_err() {
        return result;
    }

    let mut x = StarMapVecC {
        ptr: pointer,
        len: options.object_count,
    };

    generate::generate_into_slice_with_chunk_size(&mut x, options, chunk_size);

    Ok(())
}

pub fn generate(options: StarMapOptions) -> Result<Vec<StarMapEntry>, String>
{
    let mut x: Vec<StarMapEntry> = vec![
        StarMapEntry {
            x: 0f32,
            y: 0f32,
            z: 0f32,
            w: 0f32,
        };
        options.object_count as usize
    ];
    let result = options.validate();
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    generate::generate_into_slice(&mut x, options);
    Ok(x)
}

#[repr(C)]
#[derive(Clone)]
pub struct StarMapEntry
{
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

