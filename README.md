DEPRECATED - NOT IN DEVELOPMENT ANYMORE

# Engine

A 3D game engine written in Rust.

## Purpose

Building a 3D game engine from scratch. The engine includes these core systems:

- Rendering system (GPU-based 3D graphics with wgpu)
- Entity Component System (built on hecs)
- Physics engine (rapier3d integration)
- Input handling (keyboard and mouse)

## Technologies

- **wgpu**: Cross-platform GPU abstraction layer
- **hecs**: Lightweight and fast ECS library
- **rapier3d**: 3D physics simulation
- **winit**: Window creation and event handling
- **glam**: Math library (vectors, matrices, quaternions)

## Project Structure

```
src/
  core/      - Engine core and time management
  ecs/       - Entity Component System
  renderer/  - 3D rendering
  physics/   - Physics simulation
  input/     - Keyboard and mouse input
```

## Build

```bash
cargo build
```
