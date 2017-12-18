use amethyst::core::LocalTransform;
use amethyst::core::cgmath::Vector3;
use amethyst::ecs::{Entities, Fetch, Join, ReadStorage, System, WriteStorage};
use amethyst::shrev::{EventChannel, ReaderId};
use rhusics::ecs::collide::prelude2d::{BodyPose2, ContactEvent2};

use resources::ObjectType;

pub struct MovementSystem {
    contact_reader: ReaderId,
}

impl MovementSystem {
    pub fn new(contact_reader: ReaderId) -> Self {
        Self { contact_reader }
    }
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, EventChannel<ContactEvent2>>,
        ReadStorage<'a, ObjectType>,
        ReadStorage<'a, BodyPose2>,
        WriteStorage<'a, LocalTransform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, contacts, objects, poses, mut transforms) = data;

        for (pose, transform) in (&poses, &mut transforms).join() {
            *transform = LocalTransform {
                translation: Vector3::new(pose.position().x, pose.position().y, 0.),
                rotation: transform.rotation,
                scale: transform.scale,
            }
        }
    }
}
