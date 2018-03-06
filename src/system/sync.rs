use std::marker;

use amethyst_core::Transform;
use amethyst_core::cgmath::{BaseFloat, EuclideanSpace, Quaternion, Rotation, Vector3};
use rhusics_core::BodyPose;
use specs::{Join, ReadStorage, System, WriteStorage};

use sync::Convert;

/// System that copies transform information from `BodyPose` in rhusics into `Transform`
/// in amethyst.
///
/// ### Type parameters:
///
/// - `P`: Positional quantity (`Point2` or `Point3` in most scenarios).
/// - `R`: Rotational quantity (`Basis2` or `Quaternion` in most scenarios).
pub struct PoseTransformSyncSystem<P, R> {
    m: marker::PhantomData<(P, R)>,
}

impl<P, R> PoseTransformSyncSystem<P, R> {
    /// Create new system
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
        }
    }
}

impl<'a, P, R> System<'a> for PoseTransformSyncSystem<P, R>
where
    P: EuclideanSpace + Convert<Output = Vector3<f32>> + Send + Sync + 'static,
    R: Rotation<P> + Convert<Output = Quaternion<f32>> + Send + Sync + 'static,
    P::Scalar: BaseFloat,
{
    type SystemData = (ReadStorage<'a, BodyPose<P, R>>, WriteStorage<'a, Transform>);

    fn run(&mut self, data: Self::SystemData) {
        let (poses, mut transforms) = data;
        for (pose, transform) in (&poses, &mut transforms).join() {
            *transform = Transform {
                translation: pose.position().convert(),
                rotation: pose.rotation().convert(),
                scale: transform.scale,
            }
        }
    }
}
