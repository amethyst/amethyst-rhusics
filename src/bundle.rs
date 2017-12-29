use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::physics::prelude2d::{register_physics, BasicCollisionSystem2, BodyPose2,
                                       ContactEvent2, ContactResolutionSystem2, GJK2,
                                       ImpulseSolverSystem2, NextFrameSetupSystem2, SweepAndPrune2};

use resources::{Emitter, ObjectType};
use systems::{EmissionSystem, MovementSystem};

pub struct SimulationBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for SimulationBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        register_physics::<ObjectType>(world);

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
            .add(ImpulseSolverSystem2::new(), "physics_solver_system", &[])
            .add(
                MovementSystem::new(reader_2),
                "movement_system",
                &["physics_solver_system"],
            )
            .add(
                NextFrameSetupSystem2::new(),
                "next_frame_setup",
                &["physics_solver_system"],
            )
            .add(
                BasicCollisionSystem2::<BodyPose2, ObjectType>::new()
                    .with_broad_phase(SweepAndPrune2::new())
                    .with_narrow_phase(GJK2::new()),
                "basic_collision_system",
                &["next_frame_setup"],
            )
            .add(
                ContactResolutionSystem2::new(reader_1),
                "contact_resolution",
                &["basic_collision_system"],
            ))
    }
}
