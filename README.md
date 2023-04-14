# Nodium

Use Rust to create data workflows with no coding experience required.

[Github](https://github.com/cherob/nodium) | [Discord](https://discord.gg/ZTVfME7RyN)

![Latest version](https://img.shields.io/endpoint?label=crates.io&url=https%3A%2F%2Fcrates.io%2Fcrates%2Fnodium)

## Getting Started

Nodium is an easy-to-use platform designed for data analysis and automation using Rust with no coding experience required. It provides a visual node-based interface where users can create data manipulation tasks by connecting nodes in a graph. Nodium includes a plugin browser for downloading extensions that provide additional functionality, such as mathematical nodes, network nodes, language model nodes, and image model nodes. Users can create custom data workflows by connecting nodes together, making it a versatile tool for a wide range of data analysis and automation tasks.

### Prerequisites

What things you need to install the software and how to install them

* [Rust](https://www.rust-lang.org/tools/install)

### Installing

A step by step series of examples that tell you how to get a development env running
Say what the step will be

```bash
git clone https://github.com/cherob/nodium
cd nodium
cargo run
```

## Milestones

### Milestone 1: Implement the core module
- [ ] Define the `Graph`, `Node`, and `NodeType` (input, process, debug, output) types in `src/core/graph.rs` and `src/core/node.rs`.
- [ ] Implement the methods to create, add, and manipulate nodes in the graph.
- [ ] Implement the `run` method to process the graph asynchronously.

### Milestone 2: Implement the UI module
- [ ] Define the `AppController`, `PluginManager`, and `PluginBrowser` types in `src/ui/app_controller.rs`, `src/ui/plugin_manager.rs`, and `src/ui/plugin_browser.rs`.
- [ ] Implement the methods to handle user interactions and manage the application state.

### Milestone 3: Automatic extension handling
- [ ] Create a system to discover and include Nodium extensions from external Rust crates published with the `nodium-<extension-name>` pattern.
- [ ] Implement a method to rebuild the project with the included extensions.

### Milestone 4: Implement an extension browser
- [ ] Add functionality to search for and include extensions from crates.io.

## Contributing

Create a pull request and I'll review it.
Or join the [Discord](https://discord.gg/ZTVfME7RyN) and we can talk about it.

## Authors

* **Cherob** - *Initial work* - [Cherob](https://github.com/cherob)  

## Dependencies

* [egui](https://github.com/emilk/egui)
* [egui-node-graph](https://github.com/setzer22/egui_node_graph)

## Inspiration
<img src="https://camo.githubusercontent.com/c7b6e0b937295c4d2c852130814050eb0caffac5b700ead6de21df6dbf83aa82/687474703a2f2f6e6f64657265642e6f72672f696d616765732f6e6f64652d7265642d73637265656e73686f742e706e67" alt="Node-RED: Low-code programming for event-driven applications" data-canonical-src="http://nodered.org/images/node-red-screenshot.png" style="max-width: 100%;">
 
[GitHub NodeRED](https://github.com/node-red/node-red)

### Orange
  <img src="https://raw.githubusercontent.com/irgolic/orange3/README-shields/distribute/orange-example-tall.png" alt="https://raw.githubusercontent.com/irgolic/orange3/README-shields/distribute/orange-example-tall.png" class="transparent shrinkToFit" width="668" height="425">

[GitHub Orange3](https://github.com/biolab/orange3)
