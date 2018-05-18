use amethyst::core::cgmath::{Basis2, Matrix3, Quaternion, Vector3};
use collision::primitive::{Primitive2, Primitive3};
use collision::{Aabb2, Aabb3};

use super::bundle::BoxSimulationBundle;

/// Box simulation for 2D
#[allow(dead_code)]
pub type BoxSimulationBundle2 =
    BoxSimulationBundle<Primitive2<f32>, Aabb2<f32>, Basis2<f32>, f32, f32>;

/// Box simulation for 2D
#[allow(dead_code)]
pub type BoxSimulationBundle3 =
    BoxSimulationBundle<Primitive3<f32>, Aabb3<f32>, Quaternion<f32>, Vector3<f32>, Matrix3<f32>>;
