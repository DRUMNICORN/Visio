# Nodium

> [!WARNING]  
> Nodium is currently in development and is not yet ready for production use.

![GitHub all releases](https://img.shields.io/github/downloads/cherob/nodium/total?label=GitHub%20downloads&style=for-the-badge&logo=github) ![Crates.io](https://img.shields.io/crates/v/nodium?label=Crates.io&style=for-the-badge&logo=crates.io) ![Crates.io](https://img.shields.io/crates/d/nodium?label=Crates.io%20downloads&style=for-the-badge&logo=crates.io) [![Discord](https://img.shields.io/discord/1096210659588452422?label=Discord&style=for-the-badge&logo=discord)](https://discord.gg/ZTVfME7RyN)


## Your Ultimate Integrated Development Environment (IDE) for Any Project

In the future, Nodium will have evolved beyond just an Integrated Development Environment (IDE). It will have transformed into a comprehensive visual system that redefines the way you engage with programming. Its innovative interface empowers you to construct a project layout effortlessly, enabling you to concentrate on the essence and conception of your ideas, rather than becoming entangled in the intricacies of code. Nodium will continue to push the boundaries of software development, making it an indispensable tool for developers worldwide.

...is not just a tool - it's a new way of thinking about programming. It's a platform that brings together **visual design**, **AI-assisted coding**, and **community plugins** to create a truly dynamic programming environment. With Nodium, you don't just write code - you create ideas. Start your journey with Nodium today, and experience the future of development.

## Contributing

> Nodium will be an IDE focused on users building structures, and AI will create the tests and code for the user.

Create a pull request and I'll review it.  
Or join the [Discord](https://discord.gg/ZTVfME7RyN), and we can talk about it.

## Authors

- [DRUMNICORN](https://github.com/drumnicorn)

## Applications

For Nodium to be accessible, it will need a dynamic solution for using it.  
Nodium itself will be a backend architecture like [PIPEWIRE](https://pipewire.org/), and it will have different view applications.

### CLI Application

All functionalities should be available as CLI commands to make it easy for Linux users to adopt.

### Minecraft Mod

A Minecraft JAVA and BEDROCK mod would provide the opportunity to creatively start developing on the backend.

#### Control Minecraft With Arduino

[![YT Video](https://img.youtube.com/vi/xBf9fIEuX_o/0.jpg)](https://www.youtube.com/watch?v=xBf9fIEuX_o)

### Tauri (Rust): React + TypeScript

The desktop application would be the solution for primary companies and people who prefer a flat layout like [NodeRED](https://github.com/node-red/node-red).

- [Node.js](https://nodejs.org/en/download/)
- [Yarn](https://classic.yarnpkg.com/en/docs/install/#windows-stable)
- [Rust](https://www.rust-lang.org/tools/install)
- [Obsidian](https://obsidian.md/)

![image](https://github.com/drumnicorn/nodium/assets/15168847/bfe43d91-471c-458d-82a3-00421ca1b163)

### Godot 3D Game

It will be a game with plugins for reading files on PC, like audio, which will then be visible in 3D as a vinyl, for example, and in-game you can do anything with it. It will be like Garry's Mod!


## Nodium Development Roadmap

### Phase 1: Initial Integration

#### Milestone 1: Protocol and Backend Architecture

1. **Protocol Specification**
   - Define communication protocol for node interactions
   - Establish data formats and serialization methods
   - Document API endpoints for backend services

2. **Backend Architecture Design**
   - Design modular backend architecture using Rust
   - Define core components and interactions
   - Establish a plugin system with dynamic loading using libloading

3. **Initial Backend Implementation**
   - Implement core services and APIs for node management
   - Develop basic node execution engine
   - Set up database schema and integrate database support

#### Milestone 2: CLI Tool Implementation

1. **Create and Manage Flows**
   - Develop commands for flow management:
     - Add, remove, list, rename, duplicate, export, and import flows
   - Implement basic node operations and configuration

2. **Plugin Operations**
   - Enable plugin management commands:
     - List, install, remove, update, reload, and get plugin info

3. **Flow Execution and Monitoring**
   - Implement commands for flow execution:
     - Start, stop, pause, resume, monitor, log, debug, and get stats

4. **General Configuration and Setup**
   - Provide commands for general configuration and setup:
     - Set, get, reset config parameters, and initialize/update environment

#### Milestone 3: Minecraft, Godot, Tauri Integration

1. **In-game Functionality**
   - Enable file manipulation features within the game and application.
   - Create in-game interfaces for node interactions

### Phase 2: Advanced Capabilities and Plugin Ecosystem

#### Milestone 4: Expand Core Features and Plugin Ecosystem

1. **Expand Core Features**
   - Implement file nodes (Read, Write) and basic data manipulation
   - Add support for REST, GraphQL, SQL, and NoSQL databases
   - Implement basic math operations (add, subtract)

2. **Plugin Ecosystem**
   - Release initial community plugins
   - Provide tools and documentation for plugin development
   - Set up a marketplace for plugins

3. **User Feedback and Iteration**
   - Continuously gather and analyze user feedback
   - Iterate on core functionality based on feedback

#### Milestone 5: Data Analysis, IoT, and Visual Interface Enhancements

1. **Machine Learning and Data Analysis**
   - Implement machine learning processes
   - Develop tools for creating web dashboards
   - Integrate machine learning training

2. **IoT Integration**
   - Develop nodes for IoT
   - Support real-time data processing and analysis

3. **Visual Interface Enhancements**
   - Enhance the visual node-based interface
   - Implement sub-flows for modularity
   - Enable saving and reusing flows

#### Milestone 6: UX Optimizations and Mobile Integration

1. **User Experience Improvements**
   - Optimize user interface for better usability
   - Streamline workflows for smoother navigation

2. **Mobile Integration**
   - Develop mobile versions for iOS and Android platforms
   - Enable sharing of processing power for collaborative usage
   - Aim for accessibility and ease of use, making it free for all users

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

### Obsidian: Graph View

[Obsidian](https://obsidian.md/)

![Graph View](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse3.mm.bing.net%2Fth%3Fid%3DOIP.ylrThQHkTiKS5BLL3fdhQwHaGD%26pid%3DApi&f=1&ipt=7470813d96d050197f7dab3987a61d75917120a6ae854cd905bb5a84508ce579&ipo=images)

### Mindustry: Logic

[Mindustry](https://mindustrygame.github.io/)

![Logic](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fgamehelp.guru%2Fwp-content%2Fuploads%2F2020%2F02%2Fm2-min-1.png&f=1&nofb=1&ipt=d6825f99424a4e7ffb4ccc6dca9d8cd2ca920ec7db238b7e454cbe6b22e5c7c4&ipo=images)

### Factorio: Circuit Network

[Factorio](https://www.factorio.com/)

![Circuit Network](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fsimcitycoon.weebly.com%2Fuploads%2F2%2F7%2F7%2F1%2F27716059%2F5980052_orig.jpg&f=1&nofb=1&ipt=777b8d046e072d70c583098933f2c67382aa07183fcf530c5c9f4726b6d13ebc&ipo=images)

### Godot: Visual Scripting

[Godot](https://godotengine.org/)

![Visual Scripting](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fpreview.redd.it%2Fw2rpdli94h361.jpg%3Fwidth%3D1512%26format%3Dpjpg%26auto%3Dwebp%26s%3Dce4241cfa994df0b01f38c00e80c433fd53e4d84&f=1&nofb=1&ipt=1af59dbb1cdc3067ecd86dd575ebbc2f078ec66fd7265dbd2d8f82082d67d643&ipo=images)

### Houdini: VEX

[Houdini](https://www.sidefx.com/)

![VEX](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse3.mm.bing.net%2Fth%3Fid%3DOIP.7fL5PFkK6zEwdnjpS-gTBQHaFe%26pid%3DApi&f=1&ipt=ce329d2732dfcb38a39bab7da4b3ea8770b190df92908397c9563efda06adee7&ipo=images)

### Mini Metro: Metro Map

[Mini Metro](https://dinopoloclub.com/minimetro/)

![Metro Map](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fimgs.6sqft.com%2Fwp-content%2Fuploads%2F2015%2F11%2F20042557%2Fmini-metro-nyc-subway-.gif&f=1&nofb=1&ipt=eef7eebfd7a81d78dc04a18fde9eba402f017be82f853c426bf1b99f47ad3ea5&ipo=images)
