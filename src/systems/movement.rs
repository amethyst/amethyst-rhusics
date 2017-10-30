use amethyst::ecs::{Entities, Fetch, Join, ReadStorage, System, WriteStorage};
use amethyst::core::cgmath::{EuclideanSpace, InnerSpace};
use amethyst::core::{LocalTransform, Time};
use amethyst::shrev::{EventChannel, ReaderId};
use rhusics::NextFrame;
use rhusics::ecs::collide::prelude3d::{BodyPose3, ContactEvent3};

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
        Fetch<'a, EventChannel<ContactEvent3>>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, NextFrame<BodyPose3>>,
        WriteStorage<'a, BodyPose3>,
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
                            let new_pos = pose.value.position() + normal * (contact.contact.penetration_depth + 0.001);
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
                translation: pose.position().clone().to_vec(),
                rotation: pose.rotation().clone(),
                scale: transform.scale,
            }
        }

        for (velocity, pose, next) in (&velocity, &poses, &mut next).join() {
            next.value = BodyPose3::new(
                pose.position() + velocity.linear * time.delta_seconds(),
                pose.rotation().clone(),
            );
        }
    }
}
