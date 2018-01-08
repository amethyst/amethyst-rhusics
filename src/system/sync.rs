use std::marker;

use amethyst_core::LocalTransform;
use amethyst_core::cgmath::{BaseFloat, Basis2, EuclideanSpace, Point2, Point3, Quaternion,
                            Rotation, Vector3};
use rhusics_core::BodyPose;
use specs::{Join, ReadStorage, System, WriteStorage};

use sync::Convert;

pub struct PoseTransformSyncSystem<P, R> {
    m: marker::PhantomData<(P, R)>,
}

impl<P, R> PoseTransformSyncSystem<P, R> {
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
    type SystemData = (
        ReadStorage<'a, BodyPose<P, R>>,
        WriteStorage<'a, LocalTransform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (poses, mut transforms) = data;
        for (pose, transform) in (&poses, &mut transforms).join() {
            *transform = LocalTransform {
                translation: pose.position().convert(),
                rotation: pose.rotation().convert(),
                scale: transform.scale,
            }
        }
    }
}

pub type PoseTransformSyncSystem2<S> = PoseTransformSyncSystem<Point2<S>, Basis2<S>>;
pub type PoseTransformSyncSystem3<S> = PoseTransformSyncSystem<Point3<S>, Quaternion<S>>;
