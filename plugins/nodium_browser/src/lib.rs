mod nodes;
mod utils {
    mod extract;
    pub(crate) mod download;
}

use nodium_pdk::{
    NodiumPlugin, DynNodiumPlugin, DynNodiumNodeList, DynNodiumNode, StaticStr,
};

pub struct NodiumPluginBrowser;

impl NodiumPlugin for NodiumPluginBrowser {
    fn name(&self) -> &'static str {
        "browser"
    }

    fn nodes(&self) -> DynNodiumNodeList {
        let mut nodes = Vec::new();

        DynNodiumNodeList::new(nodes)
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> DynNodiumPlugin {
    DynNodiumPlugin::new(NodiumPluginBrowser)
}
