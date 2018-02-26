use amethyst::core::cgmath::{Basis2, Matrix3, Quaternion, Vector3};
use collision::{Aabb2, Aabb3};
use collision::primitive::{Primitive2, Primitive3};

use super::bundle::BoxSimulationBundle;

/// Box simulation for 2D
///
/// ### Type parameters
///
/// - `S`: Scalar (`f32` or `f64`)
#[allow(dead_code)]
pub type BoxSimulationBundle2<S> = BoxSimulationBundle<Primitive2<S>, Aabb2<S>, Basis2<S>, S, S>;

/// Box simulation for 2D
///
/// ### Type parameters
///
/// - `S`: Scalar (`f32` or `f64`)
#[allow(dead_code)]
pub type BoxSimulationBundle3<S> =
    BoxSimulationBundle<Primitive3<S>, Aabb3<S>, Quaternion<S>, Vector3<S>, Matrix3<S>>;
