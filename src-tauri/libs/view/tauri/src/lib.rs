use std::sync::Arc;

use log::{debug, error};
use nodium_app::{NodiumApp, NodiumView};
use nodium_pdk::{NodiumEvent, NodiuimNode, NodiuimWindow};
use serde_json::{from_str, to_value};
use tauri::{Manager, async_runtime::Mutex};

// TauriRenderer
#[derive(Clone)]
pub struct NodiumViewTauri {
    handle: tauri::AppHandle,
}

impl NodiumViewTauri {
    pub fn new(handle: tauri::AppHandle) -> Self {
        NodiumViewTauri { handle: handle }
    }
}

impl NodiumView for NodiumViewTauri {
    fn run(&self, app: Arc<Mutex<NodiumApp>>) -> Result<(), Box<dyn std::error::Error>> {
        debug!("running tauri renderer");
        self.handle.listen_global("event", move |event| {
            let data: String = match event.payload() {
                Some(data) => data.to_string(),
                None => {
                    error!("failed to get event payload");
                    return;
                }
            };

            let event: NodiumEvent = match from_str(&data) {
                Ok(event) => event,
                Err(e) => {
                    error!("failed to parse event: {}", e);
                    return;
                }
            };
            debug!("received event: {:?}", event);
            let app = app.clone();
            tokio::spawn(async move {
                app.lock().await.event(event.name, event.payload).await;
            });
        });
        Ok(())
    }

    fn add_window(&self, window: NodiuimWindow) -> Result<(), Box<dyn std::error::Error>> {
        debug!("adding window: {:?}", window);
        let event_payload = to_value(window)?;
        self.handle
            .app_handle()
            .emit_all("add_window", event_payload)?;
        Ok(())
    }

    fn remove_window(&self, window: NodiuimWindow) -> Result<(), Box<dyn std::error::Error>> {
        debug!("removing window: {:?}", window);
        let event_payload = to_value(window)?;
        self.handle
            .app_handle()
            .emit_all("remove_window", event_payload)?;
        Ok(())
    }

    fn update_window(&self, window: NodiuimWindow) -> Result<(), Box<dyn std::error::Error>> {
        debug!("updating window: {:?}", window);
        let event_payload = to_value(window)?;
        self.handle
            .app_handle()
            .emit_all("update_window", event_payload)?;
        Ok(())
    }

    fn add_node(&self, node: NodiuimNode) -> Result<(), Box<dyn std::error::Error>> {
        debug!("adding node: {:?}", node);
        let event_payload = to_value(node)?;
        self.handle
            .app_handle()
            .emit_all("add_node", event_payload)?;
        Ok(())
    }

    fn remove_node(&self, node: NodiuimNode) -> Result<(), Box<dyn std::error::Error>> {
        debug!("removing node: {:?}", node);
        let event_payload = to_value(node)?;
        self.handle
            .app_handle()
            .emit_all("remove_node", event_payload)?;
        Ok(())
    }

    fn add_event(&self, event: NodiumEvent) -> Result<(), Box<dyn std::error::Error>> {
        debug!("adding event: {:?}", event);
        let event_payload = to_value(event)?;
        self.handle
            .app_handle()
            .emit_all("add_event", event_payload)?;
        Ok(())
    }

    fn remove_event(&self, event: NodiumEvent) -> Result<(), Box<dyn std::error::Error>> {
        debug!("removing event: {:?}", event);
        let event_payload = to_value(event)?;
        self.handle
            .app_handle()
            .emit_all("remove_event", event_payload)?;
        Ok(())
    }

    fn update_event(&self, event: NodiumEvent) -> Result<(), Box<dyn std::error::Error>> {
        debug!("updating event: {:?}", event);
        let event_payload = to_value(event)?;
        self.handle
            .app_handle()
            .emit_all("update_event", event_payload)?;
        Ok(())
    }
}
