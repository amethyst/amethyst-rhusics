use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::physics::prelude2d::{world_physics_register_with_spatial, BasicCollisionSystem2,
                                       BodyPose2, GJK2, SweepAndPrune2, ImpulseSolverSystem2,
                                       NextFrameSetupSystem2, SpatialSortingSystem2,
                                       SpatialCollisionSystem2, ContactResolutionSystem2, ContactEvent2};
use resources::Emitter;
use emission::EmissionSystem;
use amethyst_rhusics::movement::prelude2d::*;

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

        let reader = world
            .write_resource::<EventChannel<ContactEvent2>>()
            .register_reader();

        // Register systems to handle the mission of physics objects,
        // integrate physical properties over time, resolve collisions and
        // transcribe the updated physical properties to existing transforms.
        Ok(
            dispatcher
                .add(EmissionSystem, "emission_system", &[])
                .add(
                    SpatialSortingSystem2::<BodyPose2, ()>::new(),
                    "sorting_system",
                    &["emission_system"],
                )
                .add(
                    SpatialCollisionSystem2::<BodyPose2, ()>::new()
                        .with_broad_phase(SweepAndPrune2::new())
                        .with_narrow_phase(GJK2::new()),
                    "collision_system",
                    &["sorting_system"],
                )
                .add(
                    ContactResolutionSystem2::new(reader),
                    "contact_resolution_system",
                    &["collision_system"],
                )
                .add(
                    ImpulseSolverSystem2::new(),
                    "physics_solver_system",
                    &["contact_resolution_system"],
                )
                .add(
                    NextFrameSetupSystem2::new(),
                    "next_frame_system",
                    &["contact_resolution_system"],
                )
                .add(
                    MovementSystem2::new(),
                    "movement_system",
                    &["physics_solver_system"],
                ),
        )
    }
}
