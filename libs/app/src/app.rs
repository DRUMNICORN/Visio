use std::collections::HashMap;
use crate::flow::NodiumFlow;
use tokio::sync::Mutex;
use crate::registry::NodiumRegistry;
use std::rc::Rc;

// Arc
use std::sync::Arc;

pub struct NodiumApp {
    flows: HashMap<String, Rc<Mutex<NodiumFlow>>>,
    plugins: Arc<Mutex<NodiumRegistry>>,
}

impl NodiumApp {
    pub fn new() -> Self {
        NodiumApp {
            flows: HashMap::new(),
            plugins: Arc::new(Mutex::new(NodiumRegistry::new())),
        }
    }
    
    pub async fn add_flow(&mut self, name: String) {
        let flow = NodiumFlow::new(&name);
        self.flows.insert(name, Rc::new(Mutex::new(flow)));
    }

    pub async fn remove_flow(&mut self, name: &str) -> Option<Rc<Mutex<NodiumFlow>>> {
        self.flows.remove(name)
    }

    pub async fn update_flow(
        &mut self,
        name: &str,
        new_name: String,
        new_flow: NodiumFlow,
    ) -> Result<(), String> {
        if let Some(flow) = self.flows.remove(name) {
            self.flows.insert(new_name, flow);
            Ok(())
        } else {
            Err(format!("Flow not found: {}", name))
        }
    }

    pub async fn get_flows(&self) -> HashMap<String, Rc<Mutex<NodiumFlow>>> {
        self.flows.clone()
    }
}

impl Default for NodiumApp {
    fn default() -> Self {
        Self::new()
    }
}
