use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use rhusics::ecs::physics::prelude2d::{world_physics_register_with_spatial, BasicCollisionSystem2,
                                       BodyPose2, GJK2, SweepAndPrune2, ImpulseSolverSystem2,
                                       NextFrameSetupSystem2};
use resources::Emitter;
use emission::EmissionSystem;
use amethyst_rhusics::systems::MovementSystem2;

pub struct SimulationBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for SimulationBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        // Register physics systems and component types.
        world_physics_register_with_spatial::<()>(world);

        // Register simulation component types.
        world.register::<Emitter>();

        // Register systems to handle the mission of physics objects,
        // integrate physical properties over time, resolve collisions and
        // transcribe the updated physical properties to existing transforms.
        Ok(
            dispatcher
                .add(EmissionSystem, "emission_system", &[])
                .add(
                    ImpulseSolverSystem2::new(),
                    "physics_solver_system",
                    &["emission_system"],
                )
                .add(
                    NextFrameSetupSystem2::new(),
                    "next_frame_system",
                    &["physics_solver_system"],
                )
                .add(
                    BasicCollisionSystem2::<BodyPose2>::new()
                        .with_broad_phase(SweepAndPrune2::new())

                        .with_narrow_phase(GJK2::new()),
                    "basic_collision_system",
                    &["physics_solver_system"],
                )
                .add(
                    MovementSystem2::new(),
                    "movement_system",
                    &["basic_collision_system"],
                ),
        )
    }
}
