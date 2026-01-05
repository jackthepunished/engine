//! Animation system
//!
//! Provides skeletal animation, animation clips, and playback control.

mod clip;
mod player;
mod skeleton;

pub use clip::{AnimationClip, Channel, Interpolation, Keyframe};
pub use player::{AnimationPlayer, PlaybackState};
pub use skeleton::{Bone, Skeleton, SkinningData};
