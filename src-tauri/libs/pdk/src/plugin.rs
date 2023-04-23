pub trait NodiumPlugin: Send + Sync {
  // TODO: error handling
  // type Error: std::error::Error + Send + Sync;
  fn name(&self) -> &'static str;

  // TODO: add plugin functionality
    // fn run(&self) -> Result<(), Self::Error>;
}
