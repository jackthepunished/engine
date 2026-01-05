//! Material system for meshes

use bytemuck::{Pod, Zeroable};
use glam::Vec3;

/// Material properties for rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct MaterialUniform {
    /// Base color (RGB)
    pub color: [f32; 3],
    /// Padding for alignment
    _padding1: f32,
    /// Specular strength
    pub specular: f32,
    /// Shininess factor
    pub shininess: f32,
    /// Padding for alignment
    _padding2: [f32; 2],
}

impl MaterialUniform {
    pub fn new(color: Vec3, specular: f32, shininess: f32) -> Self {
        Self {
            color: color.into(),
            _padding1: 0.0,
            specular,
            shininess,
            _padding2: [0.0; 2],
        }
    }
}

impl Default for MaterialUniform {
    fn default() -> Self {
        Self::new(Vec3::new(0.8, 0.8, 0.8), 0.5, 32.0)
    }
}

/// Material definition
#[derive(Debug, Clone)]
pub struct Material {
    /// Base color
    pub color: Vec3,
    /// Specular reflectivity (0.0 - 1.0)
    pub specular: f32,
    /// Shininess exponent
    pub shininess: f32,
}

impl Material {
    /// Create a new material with a color
    pub fn new(color: Vec3) -> Self {
        Self {
            color,
            specular: 0.5,
            shininess: 32.0,
        }
    }

    /// Create a diffuse material (no specular)
    pub fn diffuse(color: Vec3) -> Self {
        Self {
            color,
            specular: 0.0,
            shininess: 1.0,
        }
    }

    /// Create a shiny material
    pub fn shiny(color: Vec3) -> Self {
        Self {
            color,
            specular: 1.0,
            shininess: 64.0,
        }
    }

    /// Red material
    pub fn red() -> Self {
        Self::new(Vec3::new(0.9, 0.2, 0.2))
    }

    /// Green material
    pub fn green() -> Self {
        Self::new(Vec3::new(0.2, 0.9, 0.2))
    }

    /// Blue material
    pub fn blue() -> Self {
        Self::new(Vec3::new(0.2, 0.2, 0.9))
    }

    /// White material
    pub fn white() -> Self {
        Self::new(Vec3::ONE)
    }

    /// Gray material
    pub fn gray() -> Self {
        Self::new(Vec3::splat(0.5))
    }

    /// Convert to uniform data
    pub fn to_uniform(&self) -> MaterialUniform {
        MaterialUniform::new(self.color, self.specular, self.shininess)
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Vec3::new(0.8, 0.8, 0.8))
    }
}
