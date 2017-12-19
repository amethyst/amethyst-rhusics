use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::physics::prelude2d::{world_physics_register, BasicCollisionSystem2, BodyPose2,
                                       ContactEvent2, GJK2, LinearContactSolverSystem2,
                                       SweepAndPrune2};

use resources::{Emitter, ObjectType};
use systems::{EmissionSystem, MovementSystem};

pub struct SimulationBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for SimulationBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world_physics_register::<ObjectType>(world);

        world.register::<Emitter>();
        world.register::<ObjectType>();

        let reader_1 = world
            .write_resource::<EventChannel<ContactEvent2>>()
            .register_reader();
        let reader_2 = world
            .write_resource::<EventChannel<ContactEvent2>>()
            .register_reader();
        Ok(dispatcher
            .add(EmissionSystem, "emission_system", &[])
            .add(
                LinearContactSolverSystem2::new(reader_1),
                "physics_solver_system",
                &["emission_system"],
            )
            .add(
                BasicCollisionSystem2::<BodyPose2, ObjectType>::new()
                    .with_broad_phase(SweepAndPrune2::new())
                    .with_narrow_phase(GJK2::new()),
                "basic_collision_system",
                &["physics_solver_system"],
            )
            .add(
                MovementSystem::new(reader_2),
                "movement_system",
                &["basic_collision_system"],
            ))
    }
}
