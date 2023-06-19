// plugin.rs

use std::sync::Arc;


use crate::dyn_node::DynNodiumNodeList;

pub trait NodiumPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn nodes(&self) -> DynNodiumNodeList;
}

#[repr(C)]
pub struct DynNodiumPlugin {
    pointer: *const (),
    plugin: Arc<dyn NodiumPlugin>,
}

impl DynNodiumPlugin {
    pub fn new<T: 'static + NodiumPlugin>(val: T) -> Self {
        DynNodiumPlugin {
            pointer: &val as *const _ as *const (),
            plugin: Arc::new(val),
        }
    }

    pub fn name(&self) -> &'static str {
        self.plugin.name()
    }

    pub fn nodes(&self) -> DynNodiumNodeList {
        self.plugin.nodes()
    }
}

