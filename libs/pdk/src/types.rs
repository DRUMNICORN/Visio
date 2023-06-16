#[repr(C)]
#[derive(Copy, Clone)]
pub struct StaticStr {
    pointer: *const u8,
    length: usize,
}


impl StaticStr {
    pub fn new(val: &'static str) -> Self {
      StaticStr { pointer: (val.as_ptr()), length: (val.len()) }
    }

    pub fn as_str(self) -> &'static str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.pointer, self.length))
        }
    }
}