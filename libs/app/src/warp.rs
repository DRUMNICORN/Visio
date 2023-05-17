use std::hash::{Hash, Hasher};
use crates_io_api::Crate;

// TODO: Check Redundancy

pub struct WrappedCrate(pub Crate);
impl PartialEq for WrappedCrate {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

impl Eq for WrappedCrate {}
impl Hash for WrappedCrate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

impl Clone for WrappedCrate {
    fn clone(&self) -> Self {
        WrappedCrate(self.0.clone())
    }
}