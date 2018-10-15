//! Integration of amethyst and rhusics

#![warn(missing_docs)]

pub extern crate collision;
pub extern crate rhusics_core;
pub extern crate rhusics_ecs;

extern crate amethyst_core;
extern crate amethyst_renderer;

pub use self::arena::{setup_2d_arena, setup_3d_arena};
pub use self::bundle::{PhysicsBundle2, PhysicsBundle3};
pub use self::default::{
    DefaultPhysicsBundle2, DefaultPhysicsBundle3, PoseTransformSyncSystem2,
    PoseTransformSyncSystem3,
};
pub use self::pick::{pick_ray, pick_ray_screen};
pub use self::sync::{time_sync, AsTransform, Convert, PoseTransformSyncSystem};

mod arena;
mod bundle;
mod default;
mod pick;
mod sync;
