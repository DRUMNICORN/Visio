// libs/pdk/src/plugin.rs
use abi_stable::{StableAbi, sabi_trait, std_types::RString};
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::error::Error;

#[sabi_trait]
pub trait NodiumPlugin {
    fn name(&self) -> RString;
    fn version(&self) -> RString;
}