use amethyst_core::cgmath::{Basis2, Point2, Point3, Quaternion};
use collision::{Aabb2, Aabb3};
use collision::primitive::{Primitive2, Primitive3};

use bundle::{BasicPhysicsBundle2, BasicPhysicsBundle3, SpatialPhysicsBundle2,
             SpatialPhysicsBundle3};

/// Utility type for a default 2D physics setup (including collision detection).
///
/// ### Type parameters:
///
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub type DefaultBasicPhysicsBundle2<Y> = BasicPhysicsBundle2<Primitive2<f32>, Aabb2<f32>, Y>;

/// Utility type for a default 3D physics setup (including collision detection).
///
/// ### Type parameters:
///
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub type DefaultBasicPhysicsBundle3<Y> = BasicPhysicsBundle3<Primitive3<f32>, Aabb3<f32>, Y>;

/// Utility type for a default 2D physics setup (including collision detection and spatial sorting).
///
/// ### Type parameters:
///
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub type DefaultSpatialPhysicsBundle2<Y> = SpatialPhysicsBundle2<Primitive2<f32>, Aabb2<f32>, Y>;

/// Utility type for a default 3D physics setup (including collision detection and spatial sorting).
///
/// ### Type parameters:
///
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub type DefaultSpatialPhysicsBundle3<Y> = SpatialPhysicsBundle3<Primitive3<f32>, Aabb3<f32>, Y>;
