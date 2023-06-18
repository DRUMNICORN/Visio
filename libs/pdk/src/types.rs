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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyValuePair {
    key: StaticStr,
    value: StaticStr,
}

impl KeyValuePair {
    pub fn new(key: &'static str, value: &'static str) -> Self {
        KeyValuePair {
            key: StaticStr::new(key),
            value: StaticStr::new(value),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FfiSafeHashMap {
    data: *const KeyValuePair,
    length: usize,
}

impl FfiSafeHashMap {
    pub fn new(data: &'static [KeyValuePair]) -> Self {
        FfiSafeHashMap {
            data: data.as_ptr(),
            length: data.len(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&'static str> {
        unsafe {
            for i in 0..self.length {
                let pair = self.data.add(i).as_ref().unwrap();
                if pair.key.as_str() == key {
                    return Some(pair.value.as_str());
                }
            }
        }
        None
    }
}
