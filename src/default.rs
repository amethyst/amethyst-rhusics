use amethyst_core::cgmath::{Basis2, Point2, Point3, Quaternion};
use collision::{Aabb2, Aabb3};
use collision::dbvt::TreeValueWrapped;
use collision::primitive::{Primitive2, Primitive3};
use specs::Entity;

use bundle::{BasicPhysicsBundle2, BasicPhysicsBundle3};
use system::PoseTransformSyncSystem;

/// Utility type for a 2D sync system (from `BodyPose` to `Transform`).
pub type PoseTransformSyncSystem2<S> = PoseTransformSyncSystem<Point2<S>, Basis2<S>>;

/// Utility type for a 3D sync system (from `BodyPose` to `Transform`).
pub type PoseTransformSyncSystem3<S> = PoseTransformSyncSystem<Point3<S>, Quaternion<S>>;

/// Utility type for a default 2D physics setup (including collision detection).
///
/// ### Type parameters:
///
/// - `S`: scalar type (`f32` or `f64`)
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub type DefaultBasicPhysicsBundle2<S, Y> =
    BasicPhysicsBundle2<S, Primitive2<S>, Aabb2<S>, TreeValueWrapped<Entity, Aabb2<S>>, Y>;

/// Utility type for a default 3D physics setup (including collision detection).
///
/// ### Type parameters:
///
/// - `S`: scalar type (`f32` or `f64`)
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub type DefaultBasicPhysicsBundle3<S, Y> =
    BasicPhysicsBundle3<S, Primitive3<S>, Aabb3<S>, TreeValueWrapped<Entity, Aabb3<S>>, Y>;
