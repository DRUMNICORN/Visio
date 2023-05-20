# Nodium

> Nodium is currently in development and is not yet ready for production use.

Nodium is an easy-to-use data analysis and automation platform built using Rust, designed to be versatile and modular. Nodium aims to provide a user-friendly visual node-based interface for various tasks.

![GitHub all releases](https://img.shields.io/github/downloads/cherob/nodium/total?label=GitHub%20downloads&style=for-the-badge&logo=github)![Crates.io](https://img.shields.io/crates/v/nodium?label=Crates.io&style=for-the-badge&logo=crates.io)![Crates.io](https://img.shields.io/crates/d/nodium?label=Crates.io%20downloads&style=for-the-badge&logo=crates.io)![Discord](https://img.shields.io/discord/1096210659588452422?label=Discord&style=for-the-badge&logo=discord)

## Getting Started

If you want to contribute to the project, you can clone the repository and run the project locally. Think about solving an issue or adding a feature.

### Prerequisites

What things you need to install the software and how to install them

- [Rust](https://www.rust-lang.org/tools/install)

### Installing

A step by step series of examples that tell you how to get a development env running
Say what the step will be

```bash
git clone https://github.com/drumni/nodium
cd nodium
cargo run
```

#### Common Issues

Installing Missing Packages:

- "pkg-config"
- "javascriptcoregtk"

## Features

- Visual node-based interface with input and output pins.
- Dynamic loading of crates and runtime imports using libloading for plugin support
- Task-based recursive computation of nodes to execute user-created graphs
- Support for sub-flows as nodes, increasing modularity
- Save and reuse flows across multiple locations
- Async programming and a robust tasking system for performance and usability

## Milestones

### Milestone 1: Basic Nodes

- [ ] Basic Input Node (Text, Number, etc.)
- [ ] Debbugging Node (Print, Log, etc.)
- [ ] Node Connection

### Milestone 2: Basic Data Manipulation

- [ ] File Node (Read, Write, etc.)
- [ ] Basic data manipulation (filter, sort, etc.)
- [ ] APIs (REST, GraphQL, etc.)
- [ ] Databases (SQL, NoSQL, etc.)
- [ ] Basic math operations (add, subtract, etc.)

### Milestone 3: Basic Data Analysis

- [ ] Machine learning processes (text, image, audio, etc.)
- [ ] Web Dashboard creation (HTML, CSS, JS, etc.)
- [ ] Machine learning training
- [ ] IoT (MQTT, UDP, TCP, etc.)

## Contributing

Create a pull request and I'll review it.
Or join the [Discord](https://discord.gg/ZTVfME7RyN) and we can talk about it.

## Authors

- **Cherob** - _Initial work_ - [Cherob](https://github.com/cherob)

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
