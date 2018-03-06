use std::fmt::Debug;
use std::marker;

use amethyst_core::{ECSBundle, Result};
use amethyst_core::cgmath::{Basis2, Point2, Point3, Quaternion};
use collision::{Bound, ComputeBound, Contains, Discrete, Primitive, SurfaceArea, Union};
use collision::algorithm::broad_phase::{SweepAndPrune2, SweepAndPrune3};
use collision::dbvt::TreeValueWrapped;
use rhusics_core::{BodyPose, Collider, ContactEvent};
use rhusics_ecs::{SpatialCollisionSystem, SpatialSortingSystem, WithRhusics};
use rhusics_ecs::physics2d::{ContactResolutionSystem2, CurrentFrameUpdateSystem2, GJK2,
                             NextFrameSetupSystem2};
use rhusics_ecs::physics3d::{ContactResolutionSystem3, CurrentFrameUpdateSystem3, GJK3,
                             NextFrameSetupSystem3};
use shrev::EventChannel;
use specs::{DispatcherBuilder, Entity, World};

use default::{PoseTransformSyncSystem2, PoseTransformSyncSystem3};

/// Bundle for configuring 2D physics, using the spatial collision detection setup in rhusics.
///
/// ### Type parameters:
///
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (`Aabb2` for most scenarios)
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub struct SpatialPhysicsBundle2<P, B, Y> {
    m: marker::PhantomData<(P, B, Y)>,
}

impl<P, B, Y> SpatialPhysicsBundle2<P, B, Y> {
    /// Create new bundle
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
        }
    }
}

impl<'a, 'b, P, B, Y> ECSBundle<'a, 'b> for SpatialPhysicsBundle2<P, B, Y>
where
    P: Primitive<Point = Point2<f32>> + ComputeBound<B> + Send + Sync + 'static,
    B: Bound<Point = P::Point>
        + Debug
        + Clone
        + Discrete<B>
        + Union<B, Output = B>
        + Contains<B>
        + SurfaceArea<Scalar = f32>
        + Send
        + Sync
        + 'static,
    Y: Default + Collider + Send + Sync + 'static,
{
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register_physics_2d::<f32, P, B, TreeValueWrapped<Entity, B>, Y>();

        let reader = world
            .write_resource::<EventChannel<ContactEvent<Entity, Point2<f32>>>>()
            .register_reader();
        Ok(dispatcher
            .add(
                CurrentFrameUpdateSystem2::<f32>::new(),
                "physics_solver_system",
                &[],
            )
            .add(
                PoseTransformSyncSystem2::new(),
                "sync_system",
                &["physics_solver_system"],
            )
            .add(
                NextFrameSetupSystem2::<f32>::new(),
                "next_frame_setup",
                &["physics_solver_system"],
            )
            .add(
                SpatialSortingSystem::<
                    P,
                    BodyPose<Point2<f32>, Basis2<f32>>,
                    TreeValueWrapped<Entity, B>,
                    B,
                    Y,
                >::new(),
                "spatial_sorting_system",
                &["next_frame_setup"],
            )
            .add(
                SpatialCollisionSystem::<
                    P,
                    BodyPose<Point2<f32>, Basis2<f32>>,
                    (usize, TreeValueWrapped<Entity, B>),
                    B,
                    Y,
                >::new()
                    .with_broad_phase(SweepAndPrune2::<f32, B>::new())
                    .with_narrow_phase(GJK2::new()),
                "collision_system",
                &["spatial_sorting_system"],
            )
            .add(
                ContactResolutionSystem2::<f32>::new(reader),
                "contact_resolution",
                &["collision_system"],
            ))
    }
}

/// Bundle for configuring 3D physics, using the spatial collision detection setup in rhusics.
///
/// ### Type parameters:
///
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (`Aabb3` or `Sphere` for most scenarios)
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub struct SpatialPhysicsBundle3<P, B, Y> {
    m: marker::PhantomData<(P, B, Y)>,
}

impl<P, B, Y> SpatialPhysicsBundle3<P, B, Y> {
    /// Create new bundle
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
        }
    }
}

impl<'a, 'b, P, B, Y> ECSBundle<'a, 'b> for SpatialPhysicsBundle3<P, B, Y>
where
    P: Primitive<Point = Point3<f32>> + ComputeBound<B> + Send + Sync + 'static,
    B: Bound<Point = P::Point>
        + Debug
        + Clone
        + Discrete<B>
        + Union<B, Output = B>
        + Contains<B>
        + SurfaceArea<Scalar = f32>
        + Send
        + Sync
        + 'static,
    Y: Default + Collider + Send + Sync + 'static,
{
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register_physics_3d::<f32, P, B, TreeValueWrapped<Entity, B>, Y>();

        let reader = world
            .write_resource::<EventChannel<ContactEvent<Entity, Point3<f32>>>>()
            .register_reader();
        Ok(dispatcher
            .add(
                CurrentFrameUpdateSystem3::<f32>::new(),
                "physics_solver_system",
                &[],
            )
            .add(
                PoseTransformSyncSystem3::new(),
                "sync_system",
                &["physics_solver_system"],
            )
            .add(
                NextFrameSetupSystem3::<f32>::new(),
                "next_frame_setup",
                &["physics_solver_system"],
            )
            .add(
                SpatialSortingSystem::<
                    P,
                    BodyPose<Point3<f32>, Quaternion<f32>>,
                    TreeValueWrapped<Entity, B>,
                    B,
                    Y,
                >::new(),
                "spatial_sorting_system",
                &["next_frame_setup"],
            )
            .add(
                SpatialCollisionSystem::<
                    P,
                    BodyPose<Point3<f32>, Quaternion<f32>>,
                    (usize, TreeValueWrapped<Entity, B>),
                    B,
                    Y,
                >::new()
                    .with_broad_phase(SweepAndPrune3::<f32, B>::new())
                    .with_narrow_phase(GJK3::new()),
                "collision_system",
                &["spatial_sorting_system"],
            )
            .add(
                ContactResolutionSystem3::<f32>::new(reader),
                "contact_resolution",
                &["collision_system"],
            ))
    }
}
