//! Integration of amethyst and rhusics

#![warn(missing_docs)]

pub extern crate collision;
pub extern crate rhusics_core;
pub extern crate rhusics_ecs;

extern crate amethyst_core;
extern crate shrev;
extern crate specs;

pub use self::arena::{setup_2d_arena, setup_3d_arena};
pub use self::bundle::{BasicPhysicsBundle2, BasicPhysicsBundle3, SpatialPhysicsBundle2,
                       SpatialPhysicsBundle3};
pub use self::default::{DefaultBasicPhysicsBundle2, DefaultBasicPhysicsBundle3,
                        DefaultSpatialPhysicsBundle2, DefaultSpatialPhysicsBundle3,
                        PoseTransformSyncSystem2, PoseTransformSyncSystem3};
pub use self::sync::{time_sync, AsTransform, Convert};
pub use self::system::PoseTransformSyncSystem;

mod arena;
mod bundle;
mod default;
mod sync;
mod system;
