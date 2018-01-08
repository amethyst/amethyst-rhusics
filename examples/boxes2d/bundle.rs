use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::physics::prelude2d::ContactEvent2;

use resources::{Emitter, ObjectType};
use systems::{BoxDeletionSystem, EmissionSystem};

pub struct BoxSimulationBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for BoxSimulationBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<Emitter>();
        world.register::<ObjectType>();

        let reader = world
            .write_resource::<EventChannel<ContactEvent2<f32>>>()
            .register_reader();
        Ok(dispatcher.add(EmissionSystem, "emission_system", &[]).add(
            BoxDeletionSystem::new(reader),
            "deletion_system",
            &["basic_collision_system"],
        ))
    }
}
