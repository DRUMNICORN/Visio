use abi_stable::{StableAbi, std_types::RString};

pub trait NodiumPlugin : StableAbi {
  fn name(&self) -> String;
  fn version(&self) -> String;
  fn as_object(&self) -> Box<dyn NodiumPluginObject>;
  // fn run(&self) -> Result<(), Self::Error>;
}

pub trait NodiumPluginObject: Send + Sync {
  fn name(&self) -> RString;
  fn version(&self) -> RString;
  // Add any other required methods here
}

impl<T: NodiumPlugin + Send + Sync + ?Sized> NodiumPluginObject for T {
  fn name(&self) -> RString {
      NodiumPlugin::name(self).into()
  }
  fn version(&self) -> RString {
      NodiumPlugin::version(self).into()
  }
  // Implement any other required methods here
}
