//! This example program shows how to merge collision events
//! into the regular Amethyst event-handling system.
//!
use amethyst::{
    assets::AssetLoaderSystemData,
    core::{
        SystemBundle,
        EventReader,
        transform::{Transform, TransformBundle},
        ecs::{DispatcherBuilder, ReaderId},
        shrev::EventChannel,
    },
    derive::EventReader,
    ecs::{Read, SystemData},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        Camera,
        RenderingBundle,
        Texture,
        Material,
        MaterialDefaults,
        plugins::{RenderFlat3D, RenderToWindow},
        types::{DefaultBackend, Mesh},
        palette::LinSrgba,
        rendy::{
            mesh::{Position, Normal, Tangent, TexCoord},
            texture::palette::load_from_linear_rgba,
        },
        shape::Shape,
    },
    window::ScreenDimensions, utils::application_root_dir
};
use amethyst_rhusics::{
    DefaultPhysicsBundle2,
    PoseTransformSyncSystem2,
    setup_2d_arena,
    time_sync,
    collision::CollisionStrategy,
    collision::primitive::Rectangle,
    rhusics_core::{
        WorldParameters, CollisionMode, Pose, PhysicalEntity,
        collide2d::{BodyPose2, CollisionShape2},
        physics2d::{Velocity2, Mass2},
    },
    rhusics_ecs::{
        WithPhysics,
        physics2d::{CurrentFrameUpdateSystem2, NextFrameSetupSystem2},
        collide2d::ContactEvent2,
    },
};

use log::info;
use cgmath::{Point2, Basis2, EuclideanSpace, Vector2, One, Vector3, BaseNum};
use rhusics_core::Collider;
use amethyst_core::ecs::{Component, DenseVecStorage, NullStorage};
use amethyst::input::{StringBindings, BindingTypes, InputEvent};
use amethyst::winit::Event;
use amethyst::ui::UiEvent;
use std::fmt::Debug;
use crate::RacquetType::RogerFederer;


/// Shape type for collision and positioning.
type MyWorldParameters = WorldParameters<<Point2<f32> as EuclideanSpace>::Diff, <Point2<f32> as EuclideanSpace>::Scalar>;

// =================================================================================================
//
//                    Game Components
//
// =================================================================================================
/// The different types of objects in our collision system.
#[repr(u8)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum CollisionType {
    /// The collision type for arena walls.
    Wall,
    /// The collision type for event-generating racquets (boxes).
    Racquet,
    /// The collision type for event-generating balls.
    Ball,
}
impl Default for CollisionType {
    fn default() -> Self {
        CollisionType::Ball // this is the one used by
    }
}

impl Collider for CollisionType {
    /// Remove collision testing for Wall - Wall shape pairs
    fn should_generate_contacts(&self, other: &CollisionType) -> bool {
        self != other || *self != CollisionType::Wall
    }
}

// Since our data is very small, we can probably benefit from
// Dense vector storage instead of the regular one.
// For the small number of items in this example, though,
// the benefits are negligible.
impl Component for CollisionType {
    type Storage = DenseVecStorage<Self>;
}

// We'll use assorted empty components and enums to
// identify the different classes of objects in our
// system.

/// The type of racquet, i.e. the player who's yielding it
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum RacquetType {
    None,
    RogerFederer, // Wilson Pro Staff RF 97 Autograph
    SerenaWilliams, // Wilson Blade SW104
}
impl Default for RacquetType {
    fn default() -> Self { Self::None }
}
impl Component for RacquetType {
    type Storage = DenseVecStorage<Self>;
}

/// The ball, which we implement as an empty class.
///
/// We could easily have merged this
/// with "RacquetType" to be a proper "ObjectType",
/// but we wanted to illustrate the use of
/// empty components to identify objects
#[derive(Debug, Default)]
struct BallObjectType;

impl Component for BallObjectType {
    type Storage = NullStorage<Self>;
}

/// Registers the various components we need that are not automatically
/// registered by the systems.
fn register_components(world: &mut World) {
    world.register::<CollisionType>();
    world.register::<RacquetType>();
    world.register::<BallObjectType>();
}

// =================================================================================================
//
//                    Game Resources
//
// =================================================================================================
/// The default World Parameters has no gravity. Add some.
/// This also sets damping to 1.0 to turn it off.
fn add_resources(world: &mut World) {
    world.insert(MyWorldParameters::new(Vector2::new(0.0, -9.8)).with_damping(1.0));
}

// =================================================================================================
//
//                    Game Entities
//
// =================================================================================================
/// Creates a cube with size (0.5, 0.5, 0.1) and colour (1., 1., 0.7, 1.)
/// in the centre of the screen.
fn add_racquet(world: &mut World, dimensions: &ScreenDimensions, racquet_type: RacquetType) {
    let scale = Vector3::new(50., 5., 1.);
    let scale_triplet = (scale.x, scale.y, scale.z);
    let x = dimensions.width() * 0.5;
    let y = dimensions.height() * 0.1;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    // This code from amethyst/examples/rendy/main.rs
    let (mesh, albedo) = {
        let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                Shape::Cube
                    .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some(scale_triplet))
                    .into(),
                (),
            )
        });
        let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(1., 1., 0.7, 1.))
                    .into(),
                (),
            )
        });
        (mesh, albedo)
    };
    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|mtl_loader: AssetLoaderSystemData<'_, Material>| {
        mtl_loader.load_from_data(
            Material {
                albedo: albedo.clone(),
                ..material_defaults
            },
            ()
        )
    });
    world
        .create_entity()
        .with(transform)
        .with(mesh)
        .with(material)
        .with(racquet_type)
        .with_static_physical_entity(
            CollisionShape2::<f32, BodyPose2<f32>, CollisionType>::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(scale[0]*2., scale[1]*2.).into(),
                CollisionType::Racquet
            ),
            BodyPose2::<f32>::new(
                Point2::new(x, y),
                Basis2::one(),
            ),
            PhysicalEntity::default(),
            Mass2::new(10.),
        )
        .build();
}

/// Creates a ball with size (0.5, 0.5, 0.1) and colour (1., 1., 0.1, 1.)
/// in the centre of the screen.
fn add_ball(world: &mut World, dimensions: &ScreenDimensions) {
    //let scale = Vector3::new(30., 30., 1.);
    let scale = Vector3::new(20., 20., 20.);
    let scale_triplet = (scale.x, scale.y, scale.z);
    let x = dimensions.width() * 0.5;
    let y = dimensions.height() * 0.5;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    // This code from amethyst/examples/rendy/main.rs
    let (mesh, albedo) = {
        let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                // Don't know why we need scale for the sphere and then scale again for the generate...
                Shape::Sphere(scale.x as usize, scale.y as usize)
                    .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some(scale_triplet))
                    .into(),
                (),
            )
        });
        let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(1., 1., 0.1, 1.))
                    .into(),
                (),
            )
        });
        (mesh, albedo)
    };
    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|mtl_loader: AssetLoaderSystemData<'_, Material>| {
        mtl_loader.load_from_data(
            Material {
                albedo: albedo.clone(),
                ..material_defaults
            },
            ()
        )
    });
    world
        .create_entity()
        .with(transform)
        .with(mesh)
        .with(material)
        .with(BallObjectType)
        .with_dynamic_physical_entity(
            CollisionShape2::<f32, BodyPose2<f32>, CollisionType>::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(scale[0]*2., scale[1]*2.).into(),
                CollisionType::Ball
            ),
            BodyPose2::<f32>::new(
                Point2::new(x, y),
                Basis2::one(),
            ),
            Velocity2::<f32>::default(),
            PhysicalEntity::default(),
            Mass2::new(1.),
        )
        .build();
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
///
/// Note if we're rending 3d objects (e.g. cubes),
/// the camera should be set a little further back,
/// e.g. -4 or -5, so the camera doesn't end up inside the object.
fn add_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    // transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, -4.);
    // transform.prepend_rotation_y_axis(PI); // 180 degrees
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 100.);
    let camera = Camera::standard_2d(dimensions.width(), dimensions.height());
    world
        .create_entity()
        .with(camera)
        .with(transform)
        .build();
}

/// Adds all the entities this world needs.
fn add_entities(world: &mut World, dimensions: &ScreenDimensions) {
    add_camera(world, dimensions);
    add_racquet(world, dimensions, RogerFederer);
    add_ball(world, dimensions);
    //add_static_rectangle(world);
}

// =================================================================================================
//
//                    Game Systems
//
// =================================================================================================


// =================================================================================================
//
//                    Game Bundle
//
// =================================================================================================
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
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> amethyst::Result<()> {
        builder.add(CurrentFrameUpdateSystem2::<f32, BodyPose2<f32>>::new(), "current frame", &[]);
        builder.add(NextFrameSetupSystem2::<f32, BodyPose2<f32>>::new(), "next frame", &["current frame"]);
        builder.add(PoseTransformSyncSystem2::new().without_rotation(), "sync system", &[]);
        Ok(())
    }
}

// =================================================================================================
//
//                    Game Application
//
// =================================================================================================
#[derive(Clone, Debug, EventReader)]
#[reader(RhusicsStateEventReader)]
pub enum RhusicsStateEvent<T = StringBindings, S = f32>
    where
        T: BindingTypes + Clone,
        S: BaseNum + Clone + Copy + Debug + Send + Sync + 'static
{
    /// Events sent by the winit window.
    Window(Event),
    /// Events sent by the ui system.
    Ui(UiEvent),
    /// Events sent by the input system.
    Input(InputEvent<T>),
    /// Collision events
    Collision(ContactEvent2<S>),
}

// pub type Application<'a, T> = CoreApplication<'a, T, StateEvent, StateEventReader>;
pub type RhusicsApplication<'a, T> = CoreApplication<'a, T, RhusicsStateEvent, RhusicsStateEventReader>;


// =================================================================================================
//
//                    Game State
//
// =================================================================================================

/// The game state: includes a reader to read the collision events.
#[derive(Default)]
struct GameState{
    reader: Option<ReaderId<ContactEvent2<f32>>>,
}

//impl SimpleState for GameState {
impl<'a, 'b> State<GameData<'a, 'b>, RhusicsStateEvent> for GameState {
    // Here, we define hooks that will be called throughout the lifecycle of our game state.
    //
    // In this example, `on_start` is used for initializing entities
    // and `handle_state` for managing the state transitions.
    //
    // For more state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle

    /// The state is initialized with:
    /// - a camera centered in the middle of the screen.
    /// - a cube (rectangle in flat-3D) in the middle of the screen.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        register_components(world);
        add_resources(world);
        add_entities(world, &dimensions);


        self.reader = Some(world.write_resource::<EventChannel<ContactEvent2<f32>>>().register_reader());

        // let min_x = 0. - dimensions.width() / 0.5;
        // let max_x = dimensions.width() / 0.5;
        // let min_y = 0. - dimensions.height() / 0.5;
        // let max_y = dimensions.height() / 0.5;
        let min_x = 0.;
        let min_y = 0.;
        let max_x = dimensions.width();
        let max_y = dimensions.height();
        setup_2d_arena(
            Point2::new(min_x, min_y),
            Point2::new(max_x, max_y),
            (
                CollisionType::Wall,
                CollisionType::Wall,
                CollisionType::Wall,
                CollisionType::Wall,
            ),
            world,
        );
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        // mut _data: StateData<'_, GameData<'_, '_>>,
        _data: StateData<'_, GameData<'_,'_>>,
        event: RhusicsStateEvent,
    ) -> Trans<GameData<'a, 'b>, RhusicsStateEvent> {
        if let RhusicsStateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }
        else if let RhusicsStateEvent::Collision(collision) = &event {
            info!("Collision detected!");

            // The contact info is the details about the collision.
            let contact = collision.contact.clone();
            info!("  Contact = {:?}", contact);

            // The bodies are the two entities which collided
            let bodies = collision.bodies.clone();
            info!("  Bodies = {:?}", bodies);
            // We can use the world to get more info about the components attached
            // to each entity
            //
            let world = _data.world;
            let racquet_type = {
                // Using enumerated components to get info from an entity
                let racquet_types = world.read_storage::<RacquetType>();
                // We don't know which of the two bodies (if any) contains the racquet type
                let racquet_type = racquet_types.get(bodies.0)
                    .unwrap_or_else(|| racquet_types.get(bodies.1)
                        .unwrap_or_else(|| &RacquetType::None));

                match racquet_type {
                    &RacquetType::RogerFederer => "Roger Federer's racquet",
                    &RacquetType::SerenaWilliams => "Serena Williams' racquet",
                    _ => "something unknown"
                }
            };
            let ball_type = {
                // Using empty components to get the ball type
                let ball_object_types = world.read_storage::<BallObjectType>();
                if ball_object_types.get(bodies.0).is_some() || ball_object_types.get(bodies.1).is_some() {
                    "ball"
                } else {
                    "something unknown"
                }
            };
            info!("Collision between {} and {}.", ball_type, racquet_type);
        }

        // Keep going
        Trans::None
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
    fn update(&mut self, data: StateData<'_, GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, RhusicsStateEvent> {
        time_sync(data.world);
        data.data.update(data.world);
        Trans::None
    }

}

// =================================================================================================
//
//                    game setup and main
//
// =================================================================================================

/// initialize the amethyst systems and resources
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    info!("Application root is {:?}", app_root);

    let resources = app_root.join("examples/resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(DefaultPhysicsBundle2::<CollisionType>::new().with_spatial())?
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat3D::default()),
        )?;

    let mut game = RhusicsApplication::build(
        resources,
        GameState::default()
        )?
        .build(game_data)?;
    game.run();

    Ok(())
}