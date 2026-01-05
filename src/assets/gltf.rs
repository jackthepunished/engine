//! glTF 2.0 model loader
//!
//! Loads meshes, materials, and hierarchy from glTF/GLB files.

use std::path::Path;

use glam::{Quat, Vec3};

use crate::renderer::{Material, Mesh, Vertex};

/// Result type for glTF operations
pub type GltfResult<T> = Result<T, GltfError>;

/// Errors from glTF loading
#[derive(Debug, Clone)]
pub enum GltfError {
    /// Failed to open or read the file
    IoError(String),
    /// Failed to parse glTF data
    ParseError(String),
    /// Missing required data
    MissingData(String),
}

impl std::fmt::Display for GltfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {e}"),
            Self::ParseError(e) => write!(f, "Parse error: {e}"),
            Self::MissingData(e) => write!(f, "Missing data: {e}"),
        }
    }
}

impl std::error::Error for GltfError {}

/// Loaded primitive mesh data
#[derive(Debug, Clone)]
pub struct LoadedPrimitive {
    /// Vertices for this primitive
    pub vertices: Vec<Vertex>,
    /// Indices for this primitive
    pub indices: Vec<u32>,
    /// Material index (if any)
    pub material_index: Option<usize>,
}

/// Loaded mesh with primitives
#[derive(Debug, Clone)]
pub struct LoadedMesh {
    /// Mesh name
    pub name: String,
    /// All primitives in this mesh
    pub primitives: Vec<LoadedPrimitive>,
}

/// Loaded material data
#[derive(Debug, Clone)]
pub struct LoadedMaterial {
    /// Material name
    pub name: String,
    /// Base color factor (RGBA)
    pub base_color: [f32; 4],
    /// Metallic factor
    pub metallic: f32,
    /// Roughness factor
    pub roughness: f32,
    /// Base color texture path (if any)
    pub base_color_texture: Option<String>,
}

impl LoadedMaterial {
    /// Convert to engine Material
    #[must_use]
    pub fn to_material(&self) -> Material {
        Material {
            color: Vec3::new(self.base_color[0], self.base_color[1], self.base_color[2]),
            specular: 1.0 - self.roughness,
            shininess: 32.0 * (1.0 - self.roughness) + 1.0,
            use_texture: self.base_color_texture.is_some(),
        }
    }
}

/// Loaded node in the scene hierarchy
#[derive(Debug, Clone)]
pub struct LoadedNode {
    /// Node name
    pub name: String,
    /// Local translation
    pub translation: Vec3,
    /// Local rotation
    pub rotation: Quat,
    /// Local scale
    pub scale: Vec3,
    /// Mesh index (if this node has a mesh)
    pub mesh_index: Option<usize>,
    /// Child node indices
    pub children: Vec<usize>,
}

/// Complete loaded glTF scene
#[derive(Debug, Clone)]
pub struct LoadedGltf {
    /// All meshes
    pub meshes: Vec<LoadedMesh>,
    /// All materials
    pub materials: Vec<LoadedMaterial>,
    /// All nodes
    pub nodes: Vec<LoadedNode>,
    /// Root node indices
    pub root_nodes: Vec<usize>,
}

/// Load a glTF or GLB file
///
/// # Errors
///
/// Returns an error if the file cannot be loaded or parsed
pub fn load_gltf(path: impl AsRef<Path>) -> GltfResult<LoadedGltf> {
    let path = path.as_ref();

    let (document, buffers, _images) =
        gltf::import(path).map_err(|e| GltfError::IoError(e.to_string()))?;

    // Load materials
    let materials: Vec<LoadedMaterial> = document
        .materials()
        .map(|mat| {
            let pbr = mat.pbr_metallic_roughness();
            LoadedMaterial {
                name: mat.name().unwrap_or("Unnamed").to_string(),
                base_color: pbr.base_color_factor(),
                metallic: pbr.metallic_factor(),
                roughness: pbr.roughness_factor(),
                base_color_texture: pbr
                    .base_color_texture()
                    .map(|info| format!("texture_{}", info.texture().index())),
            }
        })
        .collect();

    // Load meshes
    let meshes: Vec<LoadedMesh> = document
        .meshes()
        .map(|mesh| {
            let primitives: Vec<LoadedPrimitive> = mesh
                .primitives()
                .filter_map(|prim| load_primitive(&prim, &buffers))
                .collect();

            LoadedMesh {
                name: mesh.name().unwrap_or("Unnamed").to_string(),
                primitives,
            }
        })
        .collect();

    // Load nodes
    let nodes: Vec<LoadedNode> = document
        .nodes()
        .map(|node| {
            let (translation, rotation, scale) = node.transform().decomposed();
            LoadedNode {
                name: node.name().unwrap_or("Node").to_string(),
                translation: Vec3::from_array(translation),
                rotation: Quat::from_array(rotation),
                scale: Vec3::from_array(scale),
                mesh_index: node.mesh().map(|m| m.index()),
                children: node.children().map(|c| c.index()).collect(),
            }
        })
        .collect();

    // Find root nodes (nodes that are not children of any other node)
    let root_nodes: Vec<usize> = if let Some(scene) = document.default_scene() {
        scene.nodes().map(|n| n.index()).collect()
    } else {
        // If no default scene, use all nodes without parents
        let mut is_child = vec![false; nodes.len()];
        for node in &nodes {
            for &child_idx in &node.children {
                if child_idx < is_child.len() {
                    is_child[child_idx] = true;
                }
            }
        }
        is_child
            .iter()
            .enumerate()
            .filter_map(|(i, &is_c)| if !is_c { Some(i) } else { None })
            .collect()
    };

    Ok(LoadedGltf {
        meshes,
        materials,
        nodes,
        root_nodes,
    })
}

/// Load a single primitive from a glTF mesh
fn load_primitive(
    primitive: &gltf::Primitive<'_>,
    buffers: &[gltf::buffer::Data],
) -> Option<LoadedPrimitive> {
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    // Read positions (required)
    let positions: Vec<[f32; 3]> = reader.read_positions()?.collect();

    // Read normals (optional, generate if missing)
    let normals: Vec<[f32; 3]> = reader
        .read_normals()
        .map(|iter| iter.collect())
        .unwrap_or_else(|| vec![[0.0, 1.0, 0.0]; positions.len()]);

    // Read UVs (optional)
    let uvs: Vec<[f32; 2]> = reader
        .read_tex_coords(0)
        .map(|iter| iter.into_f32().collect())
        .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);

    // Build vertices
    let vertices: Vec<Vertex> = positions
        .iter()
        .zip(normals.iter())
        .zip(uvs.iter())
        .map(|((pos, norm), uv)| Vertex {
            position: *pos,
            normal: *norm,
            uv: *uv,
        })
        .collect();

    // Read indices
    let indices: Vec<u32> = reader
        .read_indices()
        .map(|iter| iter.into_u32().collect())
        .unwrap_or_else(|| {
            // Generate sequential indices if not provided
            (0..vertices.len() as u32).collect()
        });

    Some(LoadedPrimitive {
        vertices,
        indices,
        material_index: primitive.material().index(),
    })
}

/// Helper to create engine Mesh from LoadedPrimitive
impl LoadedPrimitive {
    /// Convert to engine Mesh
    #[must_use]
    pub fn to_mesh(&self) -> Mesh {
        Mesh::from_data(self.vertices.clone(), self.indices.clone())
    }
}
