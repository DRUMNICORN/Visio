use crate::NodiumPlugin;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_struct = "PluginInterface")))]
pub struct PluginInterface {
    pub constructor: extern "C" fn() -> Box<dyn NodiumPlugin + 'static>,
    pub size: usize,
    pub drop: extern "C" fn(Box<dyn NodiumPlugin + 'static>),
}