pub trait NodiumPlugin: Send + Sync {
  fn name(&self) -> &'static str;
}
