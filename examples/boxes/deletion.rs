use std::fmt::Debug;

use amethyst::core::cgmath::{BaseFloat, EuclideanSpace};
use amethyst::ecs::{Entities, Entity, Fetch, ReadStorage, System};
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
    P: EuclideanSpace,
    P::Diff: Debug,
    P::Scalar: BaseFloat,
{
    contact_reader: ReaderId<ContactEvent<Entity, P>>,
}

impl<P> BoxDeletionSystem<P>
where
    P: EuclideanSpace,
    P::Diff: Debug,
    P::Scalar: BaseFloat,
{
    pub fn new(contact_reader: ReaderId<ContactEvent<Entity, P>>) -> Self {
        Self { contact_reader }
    }
}

impl<'a, P> System<'a> for BoxDeletionSystem<P>
where
    P: EuclideanSpace + Debug + Send + Sync + 'static,
    P::Diff: Debug + Send + Sync + 'static,
    P::Scalar: BaseFloat + Send + Sync + 'static,
{
    type SystemData = (
        Entities<'a>,
        Fetch<'a, EventChannel<ContactEvent<Entity, P>>>,
        ReadStorage<'a, ObjectType>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, contacts, objects) = data;
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
    }
}
