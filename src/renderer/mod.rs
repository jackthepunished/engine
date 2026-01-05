//! Rendering module
//!
//! 3D rendering with wgpu

mod camera;
mod material;
mod mesh;
mod renderer;

pub use camera::Camera;
pub use material::{Material, MaterialUniform};
pub use mesh::{Mesh, Vertex};
pub use renderer::{Light, ModelUniform, RenderFrame, Renderer};
