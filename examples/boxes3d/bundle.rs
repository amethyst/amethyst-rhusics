use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::physics::prelude3d::{world_physics_register, BasicCollisionSystem3, BodyPose3,
                                       ContactEvent3, GJK3, LinearSolverSystem3,
                                       SweepAndPrune3};
use resources::{Emitter, ObjectType};
use emission::EmissionSystem;
use amethyst_rhusics::systems::MovementSystem;

pub struct SimulationBundle;

impl <'a, 'b> ECSBundle<'a, 'b> for SimulationBundle {
    fn build(self, world: &mut World, dispatcher: DispatcherBuilder<'a, 'b>) -> Result<DispatcherBuilder<'a, 'b>> {
        world_physics_register(world);

        world.register::<Emitter>();
        world.register::<ObjectType>();

        let reader = world
            .read_resource::<EventChannel<ContactEvent3>>()
            .register_reader();

        Ok(
            dispatcher
                .add(EmissionSystem, "emission_system", &[])
                .add(
                    LinearSolverSystem3::new(reader.clone()),
                    "physics_solver_system",
                    &["emission_system"],
                )
                .add(
                    BasicCollisionSystem3::<BodyPose3>::new()
                        .with_broad_phase(SweepAndPrune3::new())
                        .with_narrow_phase(GJK3::new()),
                    "basic_collision_system",
                    &["physics_solver_system"],
                )
                .add(
                    MovementSystem::new(reader),
                    "movement_system",
                    &["basic_collision_system"],
                )
        )
    }
}
