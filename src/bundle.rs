use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::collide::prelude3d::{world_register, BasicCollisionSystem3, BodyPose3,
                                       ContactEvent3, GJK3, SweepAndPrune3};

use resources::{Emitter, ObjectType, Velocity};
use systems::{EmissionSystem, MovementSystem};

pub struct GameBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for GameBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world_register::<BodyPose3>(world);

        world.register::<Emitter>();
        world.register::<Velocity>();
        world.register::<ObjectType>();

        let contacts = EventChannel::<ContactEvent3>::new();
        let reader = contacts.register_reader();
        world.add_resource(contacts);

        Ok(
            dispatcher
                .add(EmissionSystem, "emission_system", &[])
                .add(
                    MovementSystem::new(reader),
                    "movement_system",
                    &["emission_system"],
                )
                .add(
                    BasicCollisionSystem3::<BodyPose3>::new()
                        .with_broad_phase(SweepAndPrune3::new())
                        .with_narrow_phase(GJK3::new()),
                    "basic_collision_system",
                    &["movement_system"],
                ),
        )
    }
}
