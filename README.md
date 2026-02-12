# Bio Rust

Bio Rust is a high-performance bioinformatics visualization and simulation engine built using the Rust programming language and the WGPU graphics API. It combines hardware-accelerated graphics with biological sequence analysis to create interactive simulations.

## Project Overview

The application performs real-time sequence analysis and provides an interactive grid-based simulation environment inspired by Conway's Game of Life. It is designed to demonstrate how biological data can be mapped to GPU-accelerated environments.

## Technical Architecture

The project is designed with a modular structure to ensure maintainability and scalability:

- **main.rs**: Orchestrates the application lifecycle, initializes the WGPU graphics state, and manages the Winit event loop.
- **universe.rs**: Contains the core simulation logic, including cell state management and the biological rules for cellular automata (Game of Life).
- **vertex.rs**: Manages the GPU vertex data structures and grid generation utilities.
- **shader.wgsl**: The WebGPU Shading Language (WGSL) code that handles vertex positioning and pixel-perfect fragment coloring directly on the hardware.

## Features

### 1. Biological Sequence Analysis
Upon startup, the engine uses the rust-bio crate to analyze a DNA sequence. It calculates key metrics such as GC-content and provides terminal-based feedback before launching the visual environment.

### 2. GPU-Accelerated Simulation
The visual environment uses WGPU to render a 20x20 high-frequency grid. The simulation runs on a separate logical tick (defaulting to 500ms) while the rendering loop continues at the display's native refresh rate for smooth interaction.

### 3. DNA-Based Seeding
The initial state of the simulation is "seeded" directly from the biological data. Guanine (G) and Cytosine (C) bases in the DNA sequence determine the starting positions of living cells, creating a unique evolutionary path for every sequence.

### 4. Interactive Environment
The grid is fully interactive. Users can modify the state of the "biological" universe in real-time:
- **Mouse Interaction**: Use the Left Click to toggle cells between alive and dead states.
- **Dynamic Buffer Updates**: Modifications are written directly to the GPU's vertex buffer using Copy-Destination (COPY_DST) transfer, ensuring zero-latency updates.
- **Keyboard Feedback**: Interactive toggles for background contrast to improve visual clarity during complex simulations.

## Prerequisites

To build and run this project, you need the following installed:
- Rust Toolchain (Cargo, Rustc)
- A GPU supporting Vulkan, Metal, or DirectX 12 (WGPU will automatically select the best available backend)

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/Vasco888888/bio-rust.git
   cd bio-rust
   ```

2. Run the application:
   ```bash
   cargo run
   ```

## Controls

- **Left Mouse Click**: Toggle cell state (Alive/Dead) in the simulation grid.
- **Any Key**: Toggle background contrast between Dim Red and Dim Blue.
- **Close Window**: Terminate the application.

## Dependencies

- **wgpu**: Low-level, cross-platform graphics API.
- **winit**: Window creation and event handling.
- **bio**: Comprehensive bioinformatics library for Rust.
- **bytemuck**: Pointer and slice casting for GPU compatibility.
- **pollster**: Simple executor for asynchronous GPU initialization.
