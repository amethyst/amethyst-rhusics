use std::fmt::Debug;

use amethyst::core::cgmath::EuclideanSpace;
use amethyst::ecs::prelude::{Entities, Entity, Read, ReadStorage, Resources, System, Write};
use amethyst::shrev::{EventChannel, ReaderId};
use rand;
use rand::Rng;
use rhusics_core::ContactEvent;

use super::{Collisions, KillRate, ObjectType};

/// Delete entities from the `World` on collision.
///
/// ### Type parameters:
///
/// - `P`: Positional quantity (`Point2` or `Point3`)
pub struct BoxDeletionSystem<P>
where
    P: EuclideanSpace<Scalar = f32> + 'static,
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
        Read<'a, KillRate>,
        ReadStorage<'a, ObjectType>,
        Write<'a, Collisions>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, contacts, kill_rate, objects, mut collisions) = data;
        collisions.0 = 0;
        for contact in contacts.read(&mut self.contact_reader.as_mut().unwrap()) {
            collisions.0 += 1;
            match (objects.get(contact.bodies.0), objects.get(contact.bodies.1)) {
                (Some(_), Some(_)) => {
                    let mut chance = rand::thread_rng().gen_range(0., 1.);
                    if chance <= kill_rate.0 {
                        match entities.delete(contact.bodies.0) {
                            Err(e) => println!("Error: {:?}", e),
                            _ => (),
                        }
                        match entities.delete(contact.bodies.1) {
                            Err(e) => println!("Error: {:?}", e),
                            _ => (),
                        }
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
