use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::physics::prelude3d::{world_physics_register_with_spatial, SpatialCollisionSystem3,
                                       BodyPose3, ContactEvent3, GJK3, ImpulseSolverSystem3,
                                       SweepAndPrune3, NextFrameSetupSystem3, SpatialSortingSystem3,
                                       ContactResolutionSystem3};
use resources::Emitter;
use emission::EmissionSystem;
use amethyst_rhusics::movement::prelude3d::*;

pub struct SimulationBundle;

impl <'a, 'b> ECSBundle<'a, 'b> for SimulationBundle {
    fn build(self, world: &mut World, dispatcher: DispatcherBuilder<'a, 'b>) -> Result<DispatcherBuilder<'a, 'b>> {
        world_physics_register_with_spatial::<()>(world);

        world.register::<Emitter>();

        let reader = world
            .write_resource::<EventChannel<ContactEvent3>>()
            .register_reader();

        Ok(
            dispatcher
                .add(EmissionSystem, "emission_system", &[])
                .add(
                    SpatialSortingSystem3::<BodyPose3, ()>::new(),
                    "sorting_system",
                    &["emission_system"]
                )
                .add(
                    SpatialCollisionSystem3::<BodyPose3, ()>::new()
                        .with_broad_phase(SweepAndPrune3::new())
                        .with_narrow_phase(GJK3::new()),
                    "collision_system",
                    &["sorting_system"],
                )
                .add(
                    ContactResolutionSystem3::new(reader),
                    "contact_resolution_system",
                    &["collision_system"],
                )
                .add(
                    ImpulseSolverSystem3::new(),
                    "physics_solver_system",
                    &["contact_resolution_system"],
                )
                .add(
                    NextFrameSetupSystem3::new(),
                    "next_frame_system",
                    &["contact_resolution_system"],
                )

                .add(
                    MovementSystem3::new(),
                    "movement_system",
                    &["physics_solver_system"],
                )
        )
    }
}
