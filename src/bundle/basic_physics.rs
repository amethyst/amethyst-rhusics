use std::marker;
use std::ops::{Add, Sub};

use amethyst_core::{ECSBundle, Result};
use amethyst_core::cgmath::{BaseFloat, Basis2, Point2, Point3, Quaternion};
use collision::{Bound, ComputeBound, Contains, Discrete, HasBound, Primitive, SurfaceArea, Union};
use collision::algorithm::broad_phase::{SweepAndPrune2, SweepAndPrune3};
use collision::dbvt::TreeValue;
use rhusics_core::{BodyPose, Collider, ContactEvent, GetId, Inertia};
use rhusics_ecs::{BasicCollisionSystem, WithRhusics};
use rhusics_ecs::physics2d::{ContactResolutionSystem2, CurrentFrameUpdateSystem2, GJK2,
                             NextFrameSetupSystem2};
use rhusics_ecs::physics3d::{ContactResolutionSystem3, CurrentFrameUpdateSystem3, GJK3,
                             NextFrameSetupSystem3};
use shrev::EventChannel;
use specs::{DispatcherBuilder, Entity, World};

use default::{PoseTransformSyncSystem2, PoseTransformSyncSystem3};

/// Bundle for configuring 2D physics, using the basic collision detection setup in rhusics.
///
/// ### Type parameters:
///
/// - `S`: Scalar (`f32` or `f64`)
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (`Aabb2` for most scenarios)
/// - `D`: Broad phase collision detection type (`TreeValueWrapped<Entity, B>` for the vast majority
///        of scenarios).
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub struct BasicPhysicsBundle2<S, P, B, D, Y> {
    m: marker::PhantomData<(S, P, B, D, Y)>,
}

impl<S, P, B, D, Y> BasicPhysicsBundle2<S, P, B, D, Y> {
    /// Create new bundle
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
        }
    }
}

impl<'a, 'b, S, P, B, D, Y> ECSBundle<'a, 'b> for BasicPhysicsBundle2<S, P, B, D, Y>
where
    P: Primitive<Point = Point2<S>> + ComputeBound<B> + Send + Sync + 'static,
    S: BaseFloat + Copy + Inertia<Orientation = Basis2<S>> + Send + Sync + 'static,
    B: Bound<Point = P::Point>
        + Clone
        + Discrete<B>
        + Union<B, Output = B>
        + Contains<B>
        + SurfaceArea<Scalar = S>
        + Send
        + Sync
        + 'static,
    D: HasBound<Bound = B>
        + From<(Entity, B)>
        + TreeValue<Bound = B>
        + GetId<Entity>
        + Send
        + Sync
        + 'static,
    Y: Default + Collider + Send + Sync + 'static,
    for<'c> &'c S: Sub<S, Output = S> + Add<S, Output = S>,
{
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register_physics_2d::<S, P, B, D, Y>();

        let reader = world
            .write_resource::<EventChannel<ContactEvent<Entity, Point2<S>>>>()
            .register_reader();
        Ok(dispatcher
            .add(
                CurrentFrameUpdateSystem2::<S>::new(),
                "physics_solver_system",
                &[],
            )
            .add(
                PoseTransformSyncSystem2::<S>::new(),
                "sync_system",
                &["physics_solver_system"],
            )
            .add(
                NextFrameSetupSystem2::<S>::new(),
                "next_frame_setup",
                &["physics_solver_system"],
            )
            .add(
                BasicCollisionSystem::<P, BodyPose<Point2<S>, Basis2<S>>, D, B, Y>::new()
                    .with_broad_phase(SweepAndPrune2::<S, B>::new())
                    .with_narrow_phase(GJK2::new()),
                "basic_collision_system",
                &["next_frame_setup"],
            )
            .add(
                ContactResolutionSystem2::<S>::new(reader),
                "contact_resolution",
                &["basic_collision_system"],
            ))
    }
}

/// Bundle for configuring 3D physics, using the basic collision detection setup in rhusics.
///
/// ### Type parameters:
///
/// - `S`: Scalar (`f32` or `f64`)
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (`Aabb3` or `Sphere` for most scenarios)
/// - `D`: Broad phase collision detection type (`TreeValueWrapped<Entity, B>` for the vast majority
///        of scenarios).
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub struct BasicPhysicsBundle3<S, P, B, D, Y> {
    m: marker::PhantomData<(S, P, B, D, Y)>,
}

impl<S, P, B, D, Y> BasicPhysicsBundle3<S, P, B, D, Y> {
    /// Create new bundle
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
        }
    }
}

impl<'a, 'b, S, P, B, D, Y> ECSBundle<'a, 'b> for BasicPhysicsBundle3<S, P, B, D, Y>
where
    P: Primitive<Point = Point3<S>> + ComputeBound<B> + Send + Sync + 'static,
    S: BaseFloat + Copy + Send + Sync + 'static,
    B: Bound<Point = P::Point>
        + Clone
        + Discrete<B>
        + Union<B, Output = B>
        + Contains<B>
        + SurfaceArea<Scalar = S>
        + Send
        + Sync
        + 'static,
    D: HasBound<Bound = B>
        + From<(Entity, B)>
        + TreeValue<Bound = B>
        + GetId<Entity>
        + Send
        + Sync
        + 'static,
    Y: Default + Collider + Send + Sync + 'static,
    for<'c> &'c S: Sub<S, Output = S> + Add<S, Output = S>,
{
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register_physics_3d::<S, P, B, D, Y>();

        let reader = world
            .write_resource::<EventChannel<ContactEvent<Entity, Point3<S>>>>()
            .register_reader();
        Ok(dispatcher
            .add(
                CurrentFrameUpdateSystem3::<S>::new(),
                "physics_solver_system",
                &[],
            )
            .add(
                PoseTransformSyncSystem3::<S>::new(),
                "sync_system",
                &["physics_solver_system"],
            )
            .add(
                NextFrameSetupSystem3::<S>::new(),
                "next_frame_setup",
                &["physics_solver_system"],
            )
            .add(
                BasicCollisionSystem::<P, BodyPose<Point3<S>, Quaternion<S>>, D, B, Y>::new()
                    .with_broad_phase(SweepAndPrune3::<S, B>::new())
                    .with_narrow_phase(GJK3::new()),
                "basic_collision_system",
                &["next_frame_setup"],
            )
            .add(
                ContactResolutionSystem3::<S>::new(reader),
                "contact_resolution",
                &["basic_collision_system"],
            ))
    }
}
