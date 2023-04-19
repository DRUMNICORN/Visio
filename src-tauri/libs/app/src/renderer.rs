
pub trait NodiumRenderer: NodiumRendererClone {
  fn run(&self, app: crate::NodiumApp) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait NodiumRendererClone {
  fn clone_box(&self) -> Box<dyn NodiumRenderer>;
}

impl<T> NodiumRendererClone for T
where
  T: 'static + NodiumRenderer + Clone,
{
  fn clone_box(&self) -> Box<dyn NodiumRenderer> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn NodiumRenderer> {
  fn clone(&self) -> Box<dyn NodiumRenderer> {
    self.clone_box()
  }
}