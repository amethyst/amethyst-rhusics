use std::fmt::Debug;
use std::marker;

use amethyst::core::{ECSBundle, Result};
use amethyst::core::cgmath::{Array, EuclideanSpace, InnerSpace, Quaternion, Rotation, Vector3,
                             Zero};
use amethyst::ecs::{DispatcherBuilder, Entity, World};
use amethyst::shrev::EventChannel;
use amethyst_rhusics::Convert;
use collision::{Bound, ComputeBound, Primitive, Union};
use rand::Rand;
use rhusics_core::{ContactEvent, Inertia};

use super::{BoxDeletionSystem, EmissionSystem, Emitter, ObjectType};

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

impl<'a, 'b, P, B, R, A, I> ECSBundle<'a, 'b> for BoxSimulationBundle<P, B, R, A, I>
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
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<Emitter<P::Point>>();
        world.register::<ObjectType>();

        let reader = world
            .write_resource::<EventChannel<ContactEvent<Entity, P::Point>>>()
            .register_reader();
        Ok(dispatcher
            .add(
                EmissionSystem::<P, B, R, A, I>::new(self.primitive),
                "emission_system",
                &[],
            )
            .add(
                BoxDeletionSystem::new(reader),
                "deletion_system",
                &["collision_system"],
            ))
    }
}
