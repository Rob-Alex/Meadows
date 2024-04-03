# MEADOWS

**MEADOWS** is a finite volume solver developed in Rust. It's designed for simulating electromagnetic wave propagation and interaction in various environments. The project aims to provide a flexible and efficient platform for both academic research and industrial applications in the field of computational electromagnetics.

## Project Objectives

- **Visualization**: Implement a visualization module to render the model of the simulation environment. I am thinking of using `gfx-rs` / `gfx-hal` or another hardware abstraction layer for graphics rendering.

## TODOs

which there are many...

### Core Features

- [ ] **Visualisation of the Model**
  - Implement a way to visually represent the model, possibly through the integration of a graphics rendering library like `gfx-rs`. This would also be used in compute shaders

### Simulation Layers

- [ ] **Layers Development**
  - [ ] Physical Layer: Display the physical attributes of the simulation environment, this includes the device under test and enviroment.
  - [ ] Electromagnetic (EM) Layer: Show electromagnetic wave interactions within the environment.
  - [ ] Further Work: I would like have the addition of a Particle Layer to display charge concentration and other microscopic phenomena that could be used to model plasmas and such.

### Model Complexity

- [ ] **Dimensionality**
  - Enable simulations to run under both 2D and 3D conditions to accommodate a variety of scenarios.
- [ ] **Symmetery/Topology**
  - Enable simulations to run with axial symmtery, or other solving conditions such as cartesian, hyperbolic, and cylindrical symmeteries.

### Interactivity

- [ ] **Model Interaction**
  - Allow user to switch between different layers.
