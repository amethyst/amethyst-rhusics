use amethyst::core::LocalTransform;
use amethyst::core::cgmath::Vector3;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use rhusics::ecs::collide::prelude2d::BodyPose2;

pub struct PoseTransformSyncSystem {}

impl PoseTransformSyncSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for PoseTransformSyncSystem {
    type SystemData = (ReadStorage<'a, BodyPose2>, WriteStorage<'a, LocalTransform>);

    fn run(&mut self, data: Self::SystemData) {
        let (poses, mut transforms) = data;
        for (pose, transform) in (&poses, &mut transforms).join() {
            *transform = LocalTransform {
                translation: Vector3::new(pose.position().x, pose.position().y, 0.),
                rotation: transform.rotation,
                scale: transform.scale,
            }
        }
    }
}
