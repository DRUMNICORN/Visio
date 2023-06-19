use nodium_pdk::node::DynNodiumNode;
use tokio::sync::mpsc;

pub struct NodiumFlow {
    pub name: String,
    pub nodes: Vec<DynNodiumNode>,
    pub connections: Vec<NodiumConnection>,
}

impl NodiumFlow {
    pub fn new(name: &str) -> NodiumFlow {
        NodiumFlow {
            name: name.to_string(),
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DynNodiumNode) {
        self.nodes.push(node);
    }

    pub fn add_connection(&mut self, sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>) {
        let connection = NodiumConnection { sender, receiver };
        self.connections.push(connection);
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct NodiumConnection {
    pub sender: mpsc::Sender<String>,
    pub receiver: mpsc::Receiver<String>,
}
