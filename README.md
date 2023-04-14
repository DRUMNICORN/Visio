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

## Inspiration

### NodeRED

[GitHub NodeRED](https://github.com/node-red/node-red)

![nodeRED](https://user-images.githubusercontent.com/15168847/232058437-213eb8a7-4eeb-4a6d-9752-60e12abf9bb7.png)

### Orange

[GitHub Orange3](https://github.com/biolab/orange3)

![orange3](https://user-images.githubusercontent.com/15168847/232058386-aee44090-4057-4427-841b-f3846b5d70a9.png)

### Blender: Geometry Nodes

[Doc Geometry Nodes](https://docs.blender.org/manual/en/latest/modeling/geometry_nodes/index.html#index-0)

![image](https://user-images.githubusercontent.com/15168847/232055166-b1cabd5e-a89b-4139-9a18-ae96e809d7e6.png)

### Unity: Shader Graph

[Doc Shader Graph](https://docs.unity3d.com/Packages/com.unity.shadergraph@14.0/manual/First-Shader-Graph.html)

![shader_graph](https://user-images.githubusercontent.com/15168847/232060906-5e22220c-be8b-45a7-92ca-cdc473f7b1cf.png)

### Drawflow

[GitHub Drawflow](https://github.com/jerosoler/Drawflow)

![drawflow](https://user-images.githubusercontent.com/15168847/232057526-63018038-440b-4a0b-baac-d366e9cba313.gif)

### Unreal Engine 5: Blueprints

[Doc Blueprints](https://docs.unrealengine.com/5.0/en-US/blueprint-tutorials-in-unreal-engine/)

![blueprints](https://user-images.githubusercontent.com/15168847/232058249-2f7ccaa3-887b-4248-8d1d-e354a52ad33a.png)
