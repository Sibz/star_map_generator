#[cfg(test)]
mod tests;

pub mod options;
pub use options::StarMapOptions;

pub fn generate_star_map_in_preallocated_memory(
    options: StarMapOptions,
    pointer: *mut u32,
) -> Result<(), String> {
    Ok(())
    //Err(String::from("re"))
}
