use egui::{CtxRef, SidePanel, TopPanel, CentralPanel};
use egui_node_graph::{GraphController, NodeGraph, NodeId, NodeLabel, PortLabel, Style};

pub struct AppController {
    graph_controller: GraphController,
}

impl AppController {
    pub fn new() -> Self {
        let mut graph_controller = GraphController::new(Style::default());
        graph_controller.populate_dummy_data(); // Remove or replace this with your own data

        Self { graph_controller }
    }

    pub fn ui(&mut self, ctx: &CtxRef) {
        TopPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Nodium");
            ui.separator();
        });

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.label("Node properties:");
            ui.separator();
            // Add your node properties UI here
        });

        CentralPanel::default().show(ctx, |ui| {
            NodeGraph::new(&mut self.graph_controller).show(ui);
        });
    }
}
