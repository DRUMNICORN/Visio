// types.rs
use std::alloc::{realloc, Layout};

use std::ptr;
#[derive(PartialEq)] // Add this line
#[repr(C)]
pub struct StaticStr {
    pointer: *const u8,
    length: usize,
}

impl StaticStr {
    pub fn new(val: &'static str) -> Self {
        StaticStr {
            pointer: val.as_ptr(),
            length: val.len(),
        }
    }

    pub fn as_str(&self) -> &'static str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.pointer, self.length))
        }
    }
}

#[repr(C)]
pub struct KeyValuePair {
    key: &'static str,
    value: &'static str,
}

impl KeyValuePair {
    pub fn new(key: &'static str, value: &'static str) -> Self {
        KeyValuePair { key, value }
    }
}

#[repr(C)]
pub struct FfiSafeHashMap {
    data: *mut KeyValuePair,
    length: usize,
    capacity: usize,
}

impl FfiSafeHashMap {
    pub fn new() -> Self {
        FfiSafeHashMap {
            data: std::ptr::null_mut(),
            length: 0,
            capacity: 0,
        }
    }

    pub fn get(&self, key: &str) -> Option<&'static str> {
        for i in 0..self.length {
            let pair = unsafe { &(*self.data.add(i)) };
            if pair.key == key {
                return Some(pair.value);
            }
        }
        None
    }

    pub fn insert(&mut self, key: StaticStr, value: StaticStr) {
        let new_pair = KeyValuePair::new(key.as_str(), value.as_str());
        let new_length = self.length + 1;

        if new_length > self.capacity {
            let new_capacity = new_length * 2;
            let layout = Layout::array::<KeyValuePair>(new_capacity).unwrap();
            let new_data = unsafe {
                realloc(
                    self.data as *mut u8,
                    layout,
                    layout.size() * new_capacity,
                ) as *mut KeyValuePair
            };

            if new_data.is_null() {
                panic!("Failed to allocate memory for FfiSafeHashMap");
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }

        unsafe {
            ptr::write(self.data.add(self.length), new_pair);
        }
        self.length = new_length;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, KeyValuePair> {
        unsafe { std::slice::from_raw_parts(self.data, self.length) }.iter()
    }
}

impl<'a> IntoIterator for &'a FfiSafeHashMap {
    type Item = (&'a str, &'static str);
    type IntoIter = std::iter::Map<std::slice::Iter<'a, KeyValuePair>, fn(&KeyValuePair) -> (&str, &'static str)>;

    fn into_iter(self) -> Self::IntoIter {
        fn convert_pair(pair: &KeyValuePair) -> (&str, &'static str) {
            (pair.key, pair.value)
        }

        self.iter().map(convert_pair as fn(&KeyValuePair) -> (&str, &'static str))
    }
}



unsafe impl Send for FfiSafeHashMap {}
unsafe impl Sync for FfiSafeHashMap {}
