use std::marker;

use amethyst_core::nalgebra as na;
use amethyst_core::specs::prelude::{Join, ReadStorage, System, World, WriteStorage};
use amethyst_core::timing::Time;
use amethyst_core::Transform;
use cgmath::{Basis2, EuclideanSpace, Point2, Point3, Quaternion, Rotation};
use rhusics_core::{BodyPose, Pose};
use rhusics_ecs::DeltaTime;

/// Utility trait for converting a transform type into an amethyst `Transform`.
pub trait AsTransform {
    /// Convert to `Transform`
    fn as_transform(&self) -> Transform;
}

/// Utility trait for converting data between types.
///
/// Primarily used for mapping data into amethysts internal data formation in `Transform`,
/// for example converting between `Point2<S>` and `Vector3<f32>` (which is used inside `Transform`).
pub trait Convert {
    /// Output type of conversion
    type Output;
    /// Convert
    fn convert(&self) -> Self::Output;
}

impl Convert for Point2<f32> {
    type Output = na::Vector3<f32>;

    fn convert(&self) -> Self::Output {
        na::Vector3::new(self.x, self.y, 0.)
    }
}

impl Convert for Point3<f32> {
    type Output = na::Vector3<f32>;

    fn convert(&self) -> Self::Output {
        na::Vector3::new(self.x, self.y, self.z)
    }
}

impl Convert for Basis2<f32> {
    type Output = na::UnitQuaternion<f32>;

    fn convert(&self) -> Self::Output {
        let matrix = na::Matrix3::new(
            self.as_ref()[0][0],
            self.as_ref()[0][1],
            0.,
            self.as_ref()[1][0],
            self.as_ref()[1][1],
            0.,
            0.,
            0.,
            1.,
        );
        na::UnitQuaternion::from_rotation_matrix(&na::Rotation3::from_matrix_unchecked(matrix))
    }
}

impl Convert for Quaternion<f32> {
    type Output = na::UnitQuaternion<f32>;

    fn convert(&self) -> Self::Output {
        na::Unit::new_normalize(na::Quaternion::new(self.s, self.v.x, self.v.y, self.v.z))
    }
}

impl<P, R> AsTransform for BodyPose<P, R>
where
    P: EuclideanSpace<Scalar = f32> + Convert<Output = na::Vector3<f32>>,
    R: Rotation<P> + Convert<Output = na::UnitQuaternion<f32>>,
{
    fn as_transform(&self) -> Transform {
        let mut t = Transform::default();
        t.set_position(self.position().convert());
        t.set_rotation(self.rotation().convert());
        t
    }
}

/// Utility function to sync time management from amethysts view of time, to rhusics view of time.
pub fn time_sync(world: &World) {
    let mut delta = world.write_resource::<DeltaTime<f32>>();
    let time = world.read_resource::<Time>();
    delta.delta_seconds = time.delta_seconds();
}

/// System that copies transform information from `BodyPose` in rhusics into `Transform`
/// in amethyst.
///
/// ### Type parameters:
///
/// - `P`: Positional quantity (`Point2<f32>` or `Point3<f32>` in most scenarios).
/// - `R`: Rotational quantity (`Basis2<f32>` or `Quaternion<f32>` in most scenarios).
pub struct PoseTransformSyncSystem<P, R> {
    m: marker::PhantomData<(P, R)>,
    translation: bool,
    rotation: bool,
}

impl<P, R> PoseTransformSyncSystem<P, R> {
    /// Create new system
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
            translation: true,
            rotation: true,
        }
    }

    /// Disable rotation sync
    pub fn without_rotation(mut self) -> Self {
        self.rotation = false;
        self
    }

    /// Disable translation sync
    pub fn without_translation(mut self) -> Self {
        self.translation = false;
        self
    }
}

use std::fmt::Debug;

impl<'a, P, R> System<'a> for PoseTransformSyncSystem<P, R>
where
    P: Debug
        + EuclideanSpace<Scalar = f32>
        + Convert<Output = na::Vector3<f32>>
        + Send
        + Sync
        + 'static,
    R: Debug + Rotation<P> + Convert<Output = na::UnitQuaternion<f32>> + Send + Sync + 'static,
{
    type SystemData = (ReadStorage<'a, BodyPose<P, R>>, WriteStorage<'a, Transform>);

    fn run(&mut self, data: Self::SystemData) {
        let (poses, mut transforms) = data;
        for (pose, transform) in (&poses, &mut transforms).join() {
            if self.translation {
                transform.set_position(pose.position().convert());
            }
            if self.rotation {
                transform.set_rotation(pose.rotation().convert());
            }
            println!("T: {:?}", transform);
            println!("P: {:?}", pose);
        }
    }
}
