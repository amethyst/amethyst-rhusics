use amethyst::ecs::{Entities, Fetch, ReadStorage, System};
use amethyst::shrev::{EventChannel, ReaderId};
use rhusics::ecs::collide::prelude2d::ContactEvent2;

use resources::ObjectType;

pub struct BoxDeletionSystem {
    contact_reader: ReaderId<ContactEvent2>,
}

impl BoxDeletionSystem {
    pub fn new(contact_reader: ReaderId<ContactEvent2>) -> Self {
        Self { contact_reader }
    }
}

impl<'a> System<'a> for BoxDeletionSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, EventChannel<ContactEvent2>>,
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
