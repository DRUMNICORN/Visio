// flows.rs
use std::collections::HashMap;
use crate::flow::NodiumFlow;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct NodiumFlows {
    flows: HashMap<String, Arc<Mutex<NodiumFlow>>>,
    // old or removed flows
    old_flows: HashMap<String, Arc<Mutex<NodiumFlow>>>,
}

impl NodiumFlows {
    pub fn new() -> Arc<Mutex<Self>> {
        let flows = NodiumFlows {
            flows: HashMap::new(),
            old_flows: HashMap::new(),
        };
        Arc::new(Mutex::new(flows))
    }

    pub async fn add_flow(&mut self, name: String, flow: NodiumFlow) {
        self.flows.insert(name, Arc::new(Mutex::new(flow)));
    }

    pub async fn remove_flow(&mut self, name: &str) -> Option<Arc<Mutex<NodiumFlow>>> {
        let removed = self.flows.remove(name)?;
        self.old_flows.insert(name.to_string(), removed.clone());
        Some(removed)
    }

    pub async fn update_flow(&mut self, name: &str, new_name: String, new_flow: NodiumFlow) -> Result<(), String> {
        if let Some(_flow) = self.flows.remove(name) {
            self.flows.insert(new_name, Arc::new(Mutex::new(new_flow)));
            Ok(())
        } else {
            Err(format!("Flow not found: {}", name))
        }
    }
}
