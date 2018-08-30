use std::fmt::Debug;
use std::marker;

use amethyst::core::cgmath::{
    Array, EuclideanSpace, InnerSpace, Quaternion, Rotation, Vector3, Zero,
};
use amethyst::core::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;
use amethyst_rhusics::Convert;
use collision::{Bound, ComputeBound, Primitive, Union};
use rand::Rand;
use rhusics_core::Inertia;

use super::{BoxDeletionSystem, EmissionSystem};

/// Bundle for box simulation.
///
/// Add spawn and deletion systems.
///
/// ### Type parameters:
///
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (usually `Aabb2`, `Aabb3` or `Sphere`)
/// - `R`: Rotational quantity (`Basis2` or `Quaternion`)
/// - `A`: Angular velocity quantity (`Scalar` or `Vector3`)
/// - `I`: Inertia tensor (`Scalar` or `Matrix3`)
pub struct BoxSimulationBundle<P, B, R, A, I> {
    primitive: P,
    m: marker::PhantomData<(B, R, A, I)>,
}

impl<P, B, R, A, I> BoxSimulationBundle<P, B, R, A, I> {
    pub fn new(primitive: P) -> Self {
        Self {
            primitive,
            m: marker::PhantomData,
        }
    }
}

impl<'a, 'b, P, B, R, A, I> SystemBundle<'a, 'b> for BoxSimulationBundle<P, B, R, A, I>
where
    B: Bound<Point = P::Point> + Union<B, Output = B> + Clone + Send + Sync + 'static,
    P: Primitive + ComputeBound<B> + Clone + Send + Sync + 'static,
    P::Point: EuclideanSpace<Scalar = f32>
        + Convert<Output = Vector3<f32>>
        + Debug
        + Send
        + Sync
        + 'static,
    <P::Point as EuclideanSpace>::Diff: Debug + Rand + InnerSpace + Array + Send + Sync + 'static,
    R: Rotation<P::Point> + Convert<Output = Quaternion<f32>> + Send + Sync + 'static,
    A: Clone + Copy + Zero + Send + Sync + 'static,
    I: Inertia + Send + Sync + 'static,
{
    fn build(self, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        dispatcher.add(
            EmissionSystem::<P, B, R, A, I>::new(self.primitive),
            "emission_system",
            &[],
        );
        dispatcher.add(
            BoxDeletionSystem::<P::Point>::new(),
            "deletion_system",
            &["contact_resolution"],
        );
        Ok(())
    }
}
