//! Integration of amethyst and rhusics

#![warn(missing_docs)]

extern crate amethyst_core;
extern crate collision;
extern crate rhusics_core;
extern crate rhusics_ecs;
extern crate shrev;
extern crate specs;

pub use self::bundle::{BasicPhysicsBundle2, BasicPhysicsBundle3, SpatialPhysicsBundle2,
                       SpatialPhysicsBundle3};
pub use self::default::{DefaultBasicPhysicsBundle2, DefaultBasicPhysicsBundle3,
                        DefaultSpatialPhysicsBundle2, DefaultSpatialPhysicsBundle3,
                        PoseTransformSyncSystem2, PoseTransformSyncSystem3};
pub use self::sync::{time_sync, AsTransform, Convert};
pub use self::system::PoseTransformSyncSystem;

mod bundle;
mod default;
mod sync;
mod system;
