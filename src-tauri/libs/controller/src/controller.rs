pub struct NodiumController {
  icons: Vec<String>,
}

// TODO: Make Controller to default feature of app

impl NodiumController {
  pub fn new() -> Self {
    NodiumController { icons: Vec::new() }
  }

  pub fn add_icon(&mut self, icon_path: String) {
      self.icons.push(icon_path);
  }

  pub fn remove_icon(&mut self, index: usize) {
      if index < self.icons.len() {
          self.icons.remove(index);
      }
  }

  pub fn icons(&self) -> &[String] {
      &self.icons
  }
}
