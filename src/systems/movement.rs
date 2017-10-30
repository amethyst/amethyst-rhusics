use amethyst::core::{LocalTransform, Time};
use amethyst::core::cgmath::{InnerSpace, Vector3};
use amethyst::ecs::{Entities, Fetch, Join, ReadStorage, System, WriteStorage};
use amethyst::shrev::{EventChannel, ReaderId};
use rhusics::NextFrame;
use rhusics::ecs::collide::prelude2d::{BodyPose2, ContactEvent2};

use resources::{ObjectType, Velocity};

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
        Fetch<'a, Time>,
        Fetch<'a, EventChannel<ContactEvent2>>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, NextFrame<BodyPose2>>,
        WriteStorage<'a, BodyPose2>,
        WriteStorage<'a, LocalTransform>,
        ReadStorage<'a, ObjectType>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, time, contacts, mut velocity, mut next, mut poses, mut transforms, objects) = data;
        match contacts.lossy_read(&mut self.contact_reader) {
            Ok(data) => for contact in data {
                match (objects.get(contact.bodies.0), objects.get(contact.bodies.1)) {
                    (Some(type_0), Some(type_1))
                        if *type_0 == ObjectType::Box || *type_1 == ObjectType::Box =>
                    {
                        if *type_0 == *type_1 {
                            match entities.delete(contact.bodies.0) {
                                Err(e) => println!("Error: {:?}", e),
                                _ => (),
                            }
                            match entities.delete(contact.bodies.1) {
                                Err(e) => println!("Error: {:?}", e),
                                _ => (),
                            }
                        } else {
                            // normal point in opposite direction of box movement
                            let (box_entity, normal) = if *type_0 == ObjectType::Box {
                                (contact.bodies.0, contact.contact.normal * -1.)
                            } else {
                                (contact.bodies.1, contact.contact.normal.clone())
                            };

                            // reflection vector is r=d−2(d⋅n)n
                            let v = velocity.get_mut(box_entity).unwrap();
                            let d = v.linear.dot(normal) * 2.;
                            if d < 0. {
                                *v = Velocity {
                                    linear: v.linear - normal * d,
                                };
                            }
                            let pose = next.get_mut(box_entity).unwrap();
                            let new_pos = pose.value.position()
                                + normal * (contact.contact.penetration_depth + 0.001);
                            pose.value.set_position(new_pos);
                        }
                    }
                    _ => {}
                }
            },
            Err(err) => println!("Error in contact read: {:?}", err),
        }

        for (next, pose, transform) in (&next, &mut poses, &mut transforms).join() {
            *pose = next.value.clone();
            *transform = LocalTransform {
                translation: Vector3::new(pose.position().x, pose.position().y, 0.),
                rotation: transform.rotation,
                scale: transform.scale,
            }
        }

        for (velocity, pose, next) in (&velocity, &poses, &mut next).join() {
            next.value = BodyPose2::new(
                pose.position() + velocity.linear * time.delta_seconds(),
                pose.rotation().clone(),
            );
        }
    }
}
