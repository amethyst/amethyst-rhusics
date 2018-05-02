use std::marker;
use std::fmt::Debug;

use amethyst_core::{Result, SystemBundle};
use amethyst_core::cgmath::{Basis2, Point2, Point3, Quaternion};
use amethyst_core::specs::prelude::{DispatcherBuilder, Entity};
use collision::{Bound, ComputeBound, Contains, Discrete, Primitive, SurfaceArea, Union};
use collision::algorithm::broad_phase::{SweepAndPrune2, SweepAndPrune3};
use collision::dbvt::TreeValueWrapped;
use rhusics_core::{BodyPose, Collider};
use rhusics_ecs::DeltaTime;
use rhusics_ecs::physics2d::{GJK2, setup_dispatch_2d};
use rhusics_ecs::physics3d::{GJK3, setup_dispatch_3d};

use default::{PoseTransformSyncSystem2, PoseTransformSyncSystem3};

/// Bundle for configuring 2D physics, using the basic collision detection setup in rhusics.
///
/// ### Type parameters:
///
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (`Aabb2` for most scenarios)
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub struct PhysicsBundle2<P, B, Y> {
    m: marker::PhantomData<(P, B, Y)>,
    spatial: bool,
}

impl<P, B, Y> PhysicsBundle2<P, B, Y> {
    /// Create new bundle
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
            spatial: false,
        }
    }

    /// Enable spatial sorting
    pub fn with_spatial(mut self) -> Self {
        self.spatial = true;
        self
    }
}

impl<'a, 'b, P, B, Y> SystemBundle<'a, 'b> for PhysicsBundle2<P, B, Y>
where
    P: Primitive<Point = Point2<f32>> + ComputeBound<B> + Send + Sync + 'static,
    B: Bound<Point = P::Point>
        + Clone
        + Discrete<B>
        + Union<B, Output = B>
        + Contains<B>
        + SurfaceArea<Scalar = f32>
        + Debug
        + Send
        + Sync
        + 'static,
    Y: Default + Collider + Send + Sync + 'static,
{
    fn build(self, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        setup_dispatch_2d::<
            f32,
            P,
            BodyPose<Point2<f32>, Basis2<f32>>,
            B,
            TreeValueWrapped<Entity, B>,
            Y,
            _,
            _,
            DeltaTime<f32>,
        >(
            dispatcher,
            SweepAndPrune2::<f32, B>::new(),
            GJK2::new(),
            self.spatial,
        );
        dispatcher.add(
            PoseTransformSyncSystem2::new(),
            "sync_system",
            &["physics_solver_system"],
        );
        Ok(())
    }
}

/// Bundle for configuring 3D physics, using the basic collision detection setup in rhusics.
///
/// ### Type parameters:
///
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (`Aabb3` or `Sphere` for most scenarios)
/// - `Y`: collision detection manager type (see `rhusics_core::Collider` for more information)
pub struct PhysicsBundle3<P, B, Y> {
    m: marker::PhantomData<(P, B, Y)>,
    spatial: bool,
}

impl<P, B, Y> PhysicsBundle3<P, B, Y> {
    /// Create new bundle
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
            spatial: false,
        }
    }

    /// Enable spatial sorting
    pub fn with_spatial(mut self) -> Self {
        self.spatial = true;
        self
    }
}

impl<'a, 'b, P, B, Y> SystemBundle<'a, 'b> for PhysicsBundle3<P, B, Y>
where
    P: Primitive<Point = Point3<f32>> + ComputeBound<B> + Send + Sync + 'static,
    B: Bound<Point = P::Point>
        + Clone
        + Discrete<B>
        + Union<B, Output = B>
        + Contains<B>
        + SurfaceArea<Scalar = f32>
        + Debug
        + Send
        + Sync
        + 'static,
    Y: Default + Collider + Send + Sync + 'static,
{
    fn build(self, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        setup_dispatch_3d::<
            f32,
            P,
            BodyPose<Point3<f32>, Quaternion<f32>>,
            B,
            TreeValueWrapped<Entity, B>,
            Y,
            _,
            _,
            DeltaTime<f32>,
        >(
            dispatcher,
            SweepAndPrune3::<f32, B>::new(),
            GJK3::new(),
            self.spatial,
        );
        dispatcher.add(
            PoseTransformSyncSystem3::new(),
            "sync_system",
            &["physics_solver_system"],
        );
        Ok(())
    }
}
