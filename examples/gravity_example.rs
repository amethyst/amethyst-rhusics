//! This is a simple example showing the physics system
//! handling gravity. A box will tumble until it hits the ground

use amethyst_core::nalgebra as na;
use amethyst_core::specs::Builder;
use amethyst_core::SystemBundle;
use amethyst_core::transform::components::Transform;
use amethyst_core::transform::TransformBundle;
use amethyst_renderer::Camera;
use amethyst_renderer::DisplayConfig;
use amethyst_renderer::DrawFlat;
use amethyst_renderer::Material;
use amethyst_renderer::MaterialDefaults;
use amethyst_renderer::Mesh;
use amethyst_renderer::Pipeline;
use amethyst_renderer::PosTex;
use amethyst_renderer::Projection;
use amethyst_renderer::RenderBundle;
use amethyst_renderer::Stage;
use amethyst_renderer::VirtualKeyCode;
use amethyst_rhusics::DefaultPhysicsBundle2;
use amethyst_rhusics::PoseTransformSyncSystem2;
use amethyst_rhusics::setup_2d_arena;
use amethyst_rhusics::time_sync;
use amethyst::Application;
use amethyst::assets::Handle;
use amethyst::assets::Loader;
use amethyst::GameData;
use amethyst::GameDataBuilder;
use amethyst::input::is_close_requested;
use amethyst::input::is_key_down;
use amethyst::Logger;
use amethyst::prelude::Config;
use amethyst::prelude::World;
use amethyst::SimpleState;
use amethyst::SimpleTrans;
use amethyst::StateData;
use amethyst::StateEvent;
use amethyst::Trans;
use amethyst::ui::DrawUi;
use amethyst::ui::UiBundle;
use cgmath::Basis2;
use cgmath::EuclideanSpace;
use cgmath::One;
use cgmath::Point2;
use cgmath::Vector2;
use collision::CollisionStrategy;
use collision::primitive::Rectangle;
use nalgebra::base::Vector3;
use rhusics_core::collide2d::BodyPose2;
use rhusics_core::collide2d::CollisionShape2;
use rhusics_core::CollisionMode;
use rhusics_core::PhysicalEntity;
use rhusics_core::physics2d::Mass2;
use rhusics_core::physics2d::Velocity2;
use rhusics_core::Pose;
use rhusics_core::WorldParameters;
use rhusics_ecs::collide2d::ContactEvent2;
use rhusics_ecs::physics2d::CurrentFrameUpdateSystem2;
use rhusics_ecs::physics2d::NextFrameSetupSystem2;
use rhusics_ecs::WithPhysics;
use shred::DispatcherBuilder;
use shrev::EventChannel;
use shrev::ReaderId;

mod boxes;
use crate::boxes::{
    ObjectType,
};

/// Shape type for collision and positioning.
type MyWorldParameters = WorldParameters<<Point2<f32> as EuclideanSpace>::Diff, <Point2<f32> as EuclideanSpace>::Scalar>;


/// Sets up our logging, display pipeline, systems and game data,
/// then runs the game.
fn run() -> Result<(), amethyst::Error> {
    // Disable logging cruft
    Logger::from_config(Default::default())
        .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
        .level_for("amethyst_renderer", amethyst::LogLevelFilter::Error)
        .start();
    let path = format!(
        "{}/examples/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    // Build pipeline
    let pipe = Pipeline::build()
        .with_stage(Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            /*
            .with_pass( DrawFlat2D::new()
                .with_transparency_settings(
                    ColorMask::all(),
                    ALPHA,
                    Some(DepthMode::LessEqualWrite),
                )
            )
            */

            .with_pass(DrawFlat::<PosTex>::new())
            .with_pass(DrawUi::new()),
        );
    // Build system bundle:
    let game_data_builder = GameDataBuilder::default()
        .with_bundle(DefaultPhysicsBundle2::<ObjectType>::new().with_spatial())?
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new().with_dep(&["sync_system"]))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::build(
        "./",
        GameState::default()
        )?.build(game_data_builder)?;

    game.run();

    Ok(())
}

/// Main method to get everything to work.
fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}

// ===================================================================
// Game Components
/// Registers the various components we need that are not automatically
/// registered by the systems.
fn register_components(world: &mut World) {
    world.register::<CollisionShape2<f32, BodyPose2<f32>, ()>>();
    world.register::<Material>();
    world.register::<ObjectType>();
}
// ===================================================================
// Game Resources

/// The default World Parameters has no gravity. Add some.
/// This also sets damping to 1.0 to turn it off.
fn add_resources(world: &mut World) {
    world.add_resource(MyWorldParameters::new(Vector2::new(0.0, -9.8)).with_damping(1.0));
}
// ===================================================================
// Game Entities
/// Adds a camera.
fn add_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(100.0);
    world.create_entity()
        .with(Camera::from(
            Projection::orthographic(
                -250.0, 250.0, -250.0, 250.0,
            )))
        .with(transform)
        .build();
}

/// Creates and returns a simple rectangle shape
/// Scaled by the given amount.
///
/// ### Parameters:
///
/// - `world`: World in which to add the shape.
/// - `scale`: Scale of the shape in x, y & z directions.
///            Scale of (1.,2.,3.) will generate a rectangle
///            2 wide, 4 high and 6 deep.
fn get_mesh(world: &mut World, scale: Vector3<f32>) -> Handle<Mesh> {
    use genmesh::generators::Cube;
    use genmesh::{MapToVertices, Triangulate, Vertices};
    let vertices = Cube::new()
        .vertex(|v| PosTex {
            position: Vector3::new(v.pos[0] * scale[0], v.pos[1] * scale[1], v.pos[2] * scale[2]),
            tex_coord: na::Vector2::new(0.1, 0.1),
        }).triangulate()
        .vertices()
        .collect::<Vec<_>>();
    world
        .read_resource::<Loader>()
        .load_from_data(vertices.into(), (), &world.read_resource())
}
/// Loads a material of the given color.
fn get_material(world: &mut World, r: f32, g: f32, b: f32) -> Material {
    let albedo = world.read_resource::<Loader>().load_from_data(
        [r, g, b, 1.0].into(),
        (),
        &world.read_resource(),
    );
    Material {
        albedo,
        ..world.read_resource::<MaterialDefaults>().0.clone()
    }
}

/// Adds a dynamic rectangle to the world, affected by gravity.
fn add_rectangle(world: &mut World) {
    let scale = Vector3::new(30., 30., 1.);
    let mesh = get_mesh(world, scale);
    let material = get_material(world, 0.1, 0.5, 0.2);
    let transform = Transform::default();
    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(transform) // will by synced to BodyPos2 by PoseTransformSyncSystem2
        .with_dynamic_physical_entity(
            CollisionShape2::<f32, BodyPose2<f32>, ObjectType>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(scale[0]*2., scale[1]*2.).into(),
            ),
            BodyPose2::<f32>::new(
                Point2::new(0., 140.),
                Basis2::one(),
            ),
            Velocity2::<f32>::default(),
            PhysicalEntity::default(),
            Mass2::new(1.),
        ).build();
}

/// Adds a static rectangle to the world, not affected by gravity.
fn add_static_rectangle(world: &mut World) {
    let scale = Vector3::new(50., 5., 1.);
    let mesh = get_mesh(world, scale);
    let material = get_material(world, 0.8, 0.4, 0.6);
    let transform = Transform::default();
    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(transform) // will by synced to BodyPos2 by PoseTransformSyncSystem2
        .with_static_physical_entity(
            CollisionShape2::<f32, BodyPose2<f32>, ObjectType>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(scale[0]*2., scale[1]*2.).into(),
            ),
            BodyPose2::<f32>::new(
                Point2::new(0., -180.),
                Basis2::one(),
            ),
            PhysicalEntity::default(),
            Mass2::new(1.),
        )
        .with(ObjectType::Box)
        .build();
}

/// Adds all the entities this world needs.
fn add_entities(world: &mut World) {
    add_camera(world);
    add_rectangle(world);
    add_static_rectangle(world);
}
// ===================================================================
// Game Systems
/*
/// A simple print system you can insert to see what is going on.
struct PrintSystem;
use amethyst_core::specs::System;
use amethyst_core::specs::Join;
use amethyst_core::specs::ReadStorage;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (
        ReadStorage<'a, PhysicalEntity<f32>>,
        ReadStorage<'a, BodyPose2<f32>>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            physical_entities,
            body_poses
        ) = data;
        for (physical_entity, body_pose) in (&physical_entities, &body_poses).join() {
            println!("Got physical entity {:?}\n    body pose: {:?}", physical_entity, body_pose);
        }
    }
}
*/

// ===================================================================
// Game Bundle
/// The game bundle.
#[derive(Default)]
struct GameBundle;

/// Adds systems to update the position of all dynamic objects based
/// on current forces and gravity.
/// (Forces can be both translational and rotational.)
///
/// The Sync system will transfer the entity's body pose data to
/// its Transform data, so the rendering system displays the body at the right
/// current position.
impl <'a, 'b>SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> amethyst::Result<()> {
        builder.add(CurrentFrameUpdateSystem2::<f32, BodyPose2<f32>>::new(), "current frame", &[]);
        builder.add(NextFrameSetupSystem2::<f32, BodyPose2<f32>>::new(), "next frame", &["current frame"]);
        builder.add(PoseTransformSyncSystem2::new(), "sync system", &[]);
        Ok(())
    }
}
// ===================================================================
// Game State

/// The game state: includes a reader to read the collision events.
#[derive(Default)]
struct GameState{
    reader: Option<ReaderId<ContactEvent2<f32>>>,
}

/// The state transitions for the game.
impl SimpleState for GameState {
    /// Starting:
    /// - initialize the components, resources and entities;
    /// - register our reader on the event channel;
    /// - set up the arena to keep the boxes inside.
    ///   (Not actually necessary in this demo as
    ///   we stop on collision between our two rectangle entities.)
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        register_components(world);
        add_resources(world);
        add_entities(world);

        self.reader = Some(world.write_resource::<EventChannel<ContactEvent2<f32>>>().register_reader());

        setup_2d_arena(
            Point2::new(-200., -200.),
            Point2::new(200., 200.),
            (
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ),
            world,
        );
    }

    /// Handle the escape key being pressed to close the game.
    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(ref event)
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) =>
                {
                    Trans::Quit
                }
            _ => Trans::None,
        }
    }
    /// Run all the systems and identify whether or not we have had a collision.
    ///
    /// The game will read the collision events from the contacts channel
    /// and stop if any events (collisions) are present.
    ///
    /// Rhusics' `SpatialCollisionSystem` is the one
    /// that generates the collision events.
    /// (Or `BasicCollisionSystem` if you remove .with_spatial()
    /// from the call to bundle `DefaultPhysicsBundle2`.)
    ///
    /// Note the call to time_sync() to keep Rhusics' time component
    /// in sync with Amethyst's.
    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        time_sync(data.world);
        data.data.update(data.world);
        let contacts_channel = data.world.read_resource::<EventChannel<ContactEvent2<f32>>>();
        let contacts = contacts_channel.read(&mut self.reader.as_mut().unwrap()).collect::<Vec<_>>();
        if contacts.len() > 0 {
            println!("Have {} contacts: Shutting down.", contacts.len());
            Trans::Quit
        } else {
            Trans::None
        }
    }
}
