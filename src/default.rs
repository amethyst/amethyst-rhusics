use amethyst_core::cgmath::{Basis2, Point2, Point3, Quaternion};
use collision::{Aabb2, Aabb3};
use collision::primitive::{Primitive2, Primitive3};

use bundle::{PhysicsBundle2, PhysicsBundle3};
use system::PoseTransformSyncSystem;

/// Utility type for a 2D sync system (from `BodyPose` to `Transform`).
pub type PoseTransformSyncSystem2 = PoseTransformSyncSystem<Point2<f32>, Basis2<f32>>;

/// Utility type for a 3D sync system (from `BodyPose` to `Transform`).
pub type PoseTransformSyncSystem3 = PoseTransformSyncSystem<Point3<f32>, Quaternion<f32>>;

/// Utility type for a default 2D physics setup (including collision detection).
///
/// ### Type parameters:
///
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub type DefaultPhysicsBundle2<Y> = PhysicsBundle2<Primitive2<f32>, Aabb2<f32>, Y>;

/// Utility type for a default 3D physics setup (including collision detection).
///
/// ### Type parameters:
///
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub type DefaultPhysicsBundle3<Y> = PhysicsBundle3<Primitive3<f32>, Aabb3<f32>, Y>;
