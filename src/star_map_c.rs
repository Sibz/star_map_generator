use crate::StarMapEntry;
use std::slice;

pub struct StarMapVecC
{
    pub ptr: *mut StarMapEntry,
    pub len: u32,
}

impl std::ops::Deref for StarMapVecC
{
    type Target = [StarMapEntry];

    fn deref(&self) -> &[StarMapEntry]
    {
        unsafe { slice::from_raw_parts(self.ptr, self.len as usize) }
    }
}

impl std::ops::DerefMut for StarMapVecC
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target
    {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len as usize) }
    }
}
