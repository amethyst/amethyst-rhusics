use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::physics::prelude2d::{world_physics_register, BasicCollisionSystem2, BodyPose2,
                                       ContactEvent2, GJK2, LinearSolverSystem2,
                                       SweepAndPrune2};
use resources::{Emitter, ObjectType};
use emission::EmissionSystem;
use amethyst_rhusics::systems::MovementSystem;

pub struct SimulationBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for SimulationBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world_physics_register(world);

        world.register::<Emitter>();
        world.register::<ObjectType>();

        let reader = world
            .read_resource::<EventChannel<ContactEvent2>>()
            .register_reader();

        Ok(
            dispatcher
                .add(EmissionSystem, "emission_system", &[])
                .add(
                    LinearSolverSystem2::new(reader.clone()),
                    "physics_solver_system",
                    &["emission_system"],
                )
                .add(
                    BasicCollisionSystem2::<BodyPose2>::new()
                        .with_broad_phase(SweepAndPrune2::new())
                        .with_narrow_phase(GJK2::new()),
                    "basic_collision_system",
                    &["physics_solver_system"],
                )
                .add(
                    MovementSystem::new(reader),
                    "movement_system",
                    &["basic_collision_system"],
                ),
        )
    }
}
