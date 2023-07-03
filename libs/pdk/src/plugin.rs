use std::sync::Arc;

use crate::node::DynNodiumNodeList;

pub trait NodiumPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn nodes(&self) -> DynNodiumNodeList;
}

pub struct DynNodiumPlugin {
    plugin: Arc<dyn NodiumPlugin>,
}

impl DynNodiumPlugin {
    pub fn new<T: 'static + NodiumPlugin + Send + Sync>(val: T) -> Self {
        DynNodiumPlugin {
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
