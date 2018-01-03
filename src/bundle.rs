use std::marker;

use amethyst::core::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::shrev::EventChannel;
use rhusics::ecs::physics::prelude2d::{register_physics, BasicCollisionSystem2, BodyPose2,
                                       Collider, ContactEvent2, ContactResolutionSystem2, GJK2,
                                       ImpulseSolverSystem2, NextFrameSetupSystem2, SweepAndPrune2};

use resources::{Emitter, ObjectType};
use systems::{BoxDeletionSystem, EmissionSystem, PoseTransformSyncSystem};

pub struct BasicPhysicsBundle2<Y> {
    m: marker::PhantomData<Y>,
}

impl<Y> BasicPhysicsBundle2<Y> {
    pub fn new() -> Self {
        Self {
            m: marker::PhantomData,
        }
    }
}

impl<'a, 'b, Y> ECSBundle<'a, 'b> for BasicPhysicsBundle2<Y>
where
    Y: Collider + Default + Send + Sync + 'static,
{
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        register_physics::<Y>(world);

        let reader = world
            .write_resource::<EventChannel<ContactEvent2>>()
            .register_reader();
        Ok(dispatcher
            .add(ImpulseSolverSystem2::new(), "physics_solver_system", &[])
            .add(
                PoseTransformSyncSystem::new(),
                "sync_system",
                &["physics_solver_system"],
            )
            .add(
                NextFrameSetupSystem2::new(),
                "next_frame_setup",
                &["physics_solver_system"],
            )
            .add(
                BasicCollisionSystem2::<BodyPose2, Y>::new()
                    .with_broad_phase(SweepAndPrune2::new())
                    .with_narrow_phase(GJK2::new()),
                "basic_collision_system",
                &["next_frame_setup"],
            )
            .add(
                ContactResolutionSystem2::new(reader),
                "contact_resolution",
                &["basic_collision_system"],
            ))
    }
}

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
            .write_resource::<EventChannel<ContactEvent2>>()
            .register_reader();
        Ok(dispatcher.add(EmissionSystem, "emission_system", &[]).add(
            BoxDeletionSystem::new(reader),
            "deletion_system",
            &["basic_collision_system"],
        ))
    }
}
