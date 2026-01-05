//! Asset management system
//!
//! Provides handle-based asset loading and storage.

mod gltf;
mod handle;
mod storage;

pub use self::gltf::{
    GltfError, GltfResult, LoadedGltf, LoadedMaterial, LoadedMesh, LoadedNode, LoadedPrimitive,
    load_gltf,
};
pub use handle::{AssetHandle, WeakAssetHandle};
pub use storage::{AssetServer, Assets};
