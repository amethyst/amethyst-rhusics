extern crate specs;

use std::fmt::Debug;

use amethyst::core::cgmath::EuclideanSpace;
use amethyst::ecs::prelude::{Entities, Entity, Read, ReadStorage, Resources, System, Join};
use amethyst::shrev::{EventChannel, ReaderId};
use rhusics_core::ContactEvent;

use super::ObjectType;

/// Delete entities from the `World` on collision.
///
/// ### Type parameters:
///
/// - `P`: Positional quantity (`Point2` or `Point3`)
pub struct BoxDeletionSystem<P>
where
    P: EuclideanSpace<Scalar = f32>,
    P::Diff: Debug,
{
    contact_reader: Option<ReaderId<ContactEvent<Entity, P>>>,
}

impl<P> BoxDeletionSystem<P>
where
    P: EuclideanSpace<Scalar = f32>,
    P::Diff: Debug,
{
    pub fn new() -> Self {
        BoxDeletionSystem {
            contact_reader: None,
        }
    }
}

impl<'a, P> System<'a> for BoxDeletionSystem<P>
where
    P: EuclideanSpace<Scalar = f32> + Debug + Send + Sync + 'static,
    P::Diff: Debug + Send + Sync + 'static,
{
    type SystemData = (
        Entities<'a>,
        Read<'a, EventChannel<ContactEvent<Entity, P>>>,
        ReadStorage<'a, ObjectType>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, contacts, objects) = data;
        for entity in (&*entities).join() {
            println!("{:?}", entity);
        }
        for contact in contacts.read(&mut self.contact_reader.as_mut().unwrap()) {
            println!("{:?}", contact);
            match (objects.get(contact.bodies.0), objects.get(contact.bodies.1)) {
                (Some(_), Some(_)) => {
                    println!("Removing entities");
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
    }

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::prelude::SystemData;
        Self::SystemData::setup(res);
        self.contact_reader = Some(
            res.fetch_mut::<EventChannel<ContactEvent<Entity, P>>>()
                .register_reader(),
        )
    }
}
