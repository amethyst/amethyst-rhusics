use amethyst::core::LocalTransform;
use amethyst::core::cgmath::Vector3;
use amethyst::ecs::{Entities, Fetch, Join, ReadStorage, System, WriteStorage};
use amethyst::shrev::{EventChannel, ReaderId};
use rhusics::ecs::collide::prelude2d::{BodyPose2, ContactEvent2};

use resources::ObjectType;

pub struct MovementSystem {
    contact_reader: ReaderId<ContactEvent2>,
}

impl MovementSystem {
    pub fn new(contact_reader: ReaderId<ContactEvent2>) -> Self {
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
        for contact in contacts.read(&mut self.contact_reader) {
            println!("{:?}", contact);
            match (objects.get(contact.bodies.0), objects.get(contact.bodies.1)) {
                (Some(_), Some(_)) => {
                    match entities.delete(contact.bodies.0) {
                        Err(e) => println!("Error: {:?}", e),
                        _ => (),
                    }
                    match entities.delete(contact.bodies.1) {
                        Err(e) => println!("Error: {:?}", e),
                        _ => (),
                    }
                }
                _ => {}
            }
        }

        for (pose, transform) in (&poses, &mut transforms).join() {
            *transform = LocalTransform {
                translation: Vector3::new(pose.position().x, pose.position().y, 0.),
                rotation: transform.rotation,
                scale: transform.scale,
            }
        }
    }
}
