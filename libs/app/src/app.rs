use std::collections::HashMap;
use crate::flow::NodiumFlow;
use tokio::sync::Mutex;
use crate::registry::NodiumRegistry;

// Arc
use std::sync::Arc;

pub struct NodiumApp {
    pub flows: Arc<Mutex<HashMap<String, NodiumFlow>>>,
    pub plugins: Arc<Mutex<NodiumRegistry>>,
}

impl NodiumApp {
    pub fn new() -> Self {
        NodiumApp {
            flows: Arc::new(Mutex::new(HashMap::new())),
            plugins: Arc::new(Mutex::new(NodiumRegistry::new())),
        }
    }
    
    pub async fn add_flow(&mut self, name: String) {
        let flow = NodiumFlow::new(&name);
        self.flows.lock().await.insert(name, flow);
    }

    pub async fn remove_flow(&mut self, name: &str) -> Option<NodiumFlow> {
        self.flows.lock().await.remove(name)
    }

    pub async fn update_flow(
        &mut self,
        name: &str,
        new_name: String,
        _new_flow: NodiumFlow,
    ) -> Result<(), String> {
        if let Some(flow) = self.flows.lock().await.remove(name) {
            self.flows.lock().await.insert(new_name, flow);
            Ok(())
        } else {
            Err(format!("Flow not found: {}", name))
        }
    }

    pub async fn get_flows(&self) -> Arc<Mutex<HashMap<String, NodiumFlow>>> {
        self.flows.clone()
    }
}

impl Default for NodiumApp {
    fn default() -> Self {
        Self::new()
    }
}
