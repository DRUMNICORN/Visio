

impl CratesBrowserWindow {
  pub fn new(crates_service: Arc<CratesService>) -> Self {
      let content = NodiumUiComponent::Tabs(vec![
          NodiumUiComponent::InputField("Search".to_string(), "search".to_string()),
          NodiumUiComponent::Button("Update".to_string(), "update".to_string()),
      ]);

      CratesBrowserWindow {
          crates_service,
          content,
      }
  }
}


impl NodiumWindow for CratesBrowserWindow {
  fn name(&self) -> String {
      "CratesBrowserWindow".to_string()
  }

  fn icon(&self) -> String {
      "icon.png".to_string()
  }

  fn title(&self) -> String {
      "Crates Browser".to_string()
  }

  fn content(&self) -> NodiumUiComponent {
      self.content.clone()
  }

  fn on_event(&mut self, event_name: &str, _event_data: &str) {
      debug!("CratesBrowserWindow received event: {}", event_name);
      // match event_name {
      //     "update" => {
      //         if let Err(e) = self.crates_service.fetch_crates() {
      //             debug!("Error fetching crates: {}", e);
      //         } else {
      //             let crates = self.crates_service.crates();
      //             // Update the table with the new crates data
      //             // self.content = NodiumUiComponent::Table(crates);
      //         }
      //         }
      //     }
      //     "install" => {
      //         // Install the crate
      //         debug!("Install the crate");
      //     }
      //     _ => {}
      // }
  }
}
