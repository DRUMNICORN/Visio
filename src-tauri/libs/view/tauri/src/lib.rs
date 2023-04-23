use std::sync::Arc;
use tokio::sync::Mutex;

use log::{debug, error};
use nodium_app::NodiumView;
use nodium_events::NodiumEventBus;
use nodium_pdk::{NodiumEvent, NodiumNode, NodiumWindow};
use serde_json::{from_str, to_value};
use tauri::{AppHandle, Manager};

// Tauri view
#[derive(Clone)]
pub struct NodiumViewTauri {
    handle: AppHandle,
    event_bus: Arc<Mutex<NodiumEventBus>>,
}

impl NodiumViewTauri {
    pub fn new(handle: AppHandle, event_bus: Arc<Mutex<NodiumEventBus>>) -> Self {
        NodiumViewTauri {
            handle: handle,
            event_bus: event_bus,
        }
    }
}

impl NodiumView for NodiumViewTauri {
    fn run(
        &self,
        // async result
    ) -> Result<(), Box<dyn std::error::Error>> {
        // debug!("running tauri view");

        // // event manager will listen for events to frontend // callback in a box
        // let event_bus_1 = self.event_bus.clone();
        
        // // register event
        // let event_log = self.event_bus.lock().await;


        // event_log.register(
        //     "event",
        //     Box::new(move |payload| {
        //         let event_bus_1 = event_bus_1.clone();
        //         debug!("received event: {}", payload);
        //         let event: NodiumEvent = match from_str(&payload) {
        //             Ok(event) => event,
        //             Err(e) => {
        //                 error!("failed to parse event: {}", e);
        //                 return;
        //             }
        //         };
        //         debug!("received event: {:?}", event);
        //         let event = NodiumEvent::new(&event.name, event.payload);
        //         // event_callback(event);+
        //         let event_json_str = serde_json::to_string(&event).unwrap();

        //         tokio::spawn(async move {
        //             event_bus_1.lock().await.emit("event", event_json_str);
        //         });
        //     }),
        // );
        // let event_bus_2 = self.event_bus.clone();

        // self.handle.listen_global("event", move |event| {
        //     let event_bus_2 = event_bus_2.clone();
        //     debug!("received event: {:?}", event);
        //     let data: String = match event.payload() {
        //         Some(data) => data.to_string(),
        //         None => {
        //             error!("failed to get event payload");
        //             return;
        //         }
        //     };
        //     debug!("received event payload: {}", data);
        //     let event: NodiumEvent = match from_str(&data) {
        //         Ok(event) => event,
        //         Err(e) => {
        //             error!("failed to parse event: {}", e);
        //             return;
        //         }
        //     };
        //     debug!("received event: {:?}", event);
        //     let event = NodiumEvent::new(&event.name, event.payload);
        //     // event_callback(event);
        //     let event_json_str = serde_json::to_string(&event).unwrap();
        //     tokio::spawn(async move {
        //         event_bus_2.lock().await.emit("event", event_json_str);
        //     });
        // });
        Ok(())
    }

    fn add_window(&self, window: Box<dyn NodiumWindow>) -> Result<(), Box<dyn std::error::Error>> {
        debug!("adding window: {:?}", window.serialize());
        let event_payload = to_value(window.serialize())?;
        self.handle
            .app_handle()
            .emit_all("add_window", event_payload)?;
        Ok(())
    }

    fn remove_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("removing window: {:?}", window.serialize());
        let event_payload = to_value(window.serialize())?;
        self.handle
            .app_handle()
            .emit_all("remove_window", event_payload)?;
        Ok(())
    }

    fn update_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("updating window: {:?}", window.serialize());
        let event_payload = to_value(window.serialize())?;
        self.handle
            .app_handle()
            .emit_all("update_window", event_payload)?;
        Ok(())
    }

    fn add_node(&self, node: NodiumNode) -> Result<(), Box<dyn std::error::Error>> {
        debug!("adding node: {:?}", node);
        let event_payload = to_value(node)?;
        self.handle
            .app_handle()
            .emit_all("add_node", event_payload)?;
        Ok(())
    }

    fn remove_node(&self, node: NodiumNode) -> Result<(), Box<dyn std::error::Error>> {
        debug!("removing node: {:?}", node);
        let event_payload = to_value(node)?;
        self.handle
            .app_handle()
            .emit_all("remove_node", event_payload)?;
        Ok(())
    }
}
