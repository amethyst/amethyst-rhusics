use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use amethyst::utils::fps_counter::{FPSCounter, FPSCounterSystem};
use rhusics::ecs::collide::prelude2d::{world_register, BasicCollisionSystem2, BodyPose2,
                                       ContactEvent2, GJK2, SweepAndPrune2};

use resources::{Emitter, ObjectType, Velocity};
use systems::{EmissionSystem, MovementSystem};

pub struct SimulationBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for SimulationBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world_register::<BodyPose2>(world);

        world.register::<Emitter>();
        world.register::<Velocity>();
        world.register::<ObjectType>();

        let contacts = EventChannel::<ContactEvent2>::new();
        let reader = contacts.register_reader();
        world.add_resource(contacts);
        world.add_resource(FPSCounter::new(20));

        Ok(
            dispatcher
                .add(FPSCounterSystem, "", &[])
                .add(EmissionSystem, "emission_system", &[])
                .add(
                    MovementSystem::new(reader),
                    "movement_system",
                    &["emission_system"],
                )
                .add(
                    BasicCollisionSystem2::<BodyPose2>::new()
                        .with_broad_phase(SweepAndPrune2::new())
                        .with_narrow_phase(GJK2::new()),
                    "basic_collision_system",
                    &["movement_system"],
                ),
        )
    }
}
