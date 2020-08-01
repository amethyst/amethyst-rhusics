use amethyst::{
    assets::AssetLoaderSystemData,
    core::{
        SystemBundle,
        transform::{Transform, TransformBundle},
        ecs::{DispatcherBuilder, ReaderId},
        shrev::EventChannel,
    },
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
use cgmath::{Point2, Basis2, EuclideanSpace, Vector2, One, Vector3};

mod boxes;
use boxes::ObjectType;

/// Shape type for collision and positioning.
type MyWorldParameters = WorldParameters<<Point2<f32> as EuclideanSpace>::Diff, <Point2<f32> as EuclideanSpace>::Scalar>;

// =================================================================================================
//
//                    Game Components
//
// =================================================================================================
/// Registers the various components we need that are not automatically
/// registered by the systems.
fn register_components(world: &mut World) {
    world.register::<CollisionShape2<f32, BodyPose2<f32>, ()>>();
    //world.register::<Material>();
    world.register::<ObjectType>();
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
fn add_rectangle(world: &mut World, dimensions: &ScreenDimensions) {
    let scale = Vector3::new(30., 30., 1.);
    let scale_triplet = (scale.x, scale.y, scale.z);
    let x = dimensions.width() * 0.5;
    let y = dimensions.height() * 0.5;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);
    // Scale is needed if you generate the mesh using genmesh (initialize_mesh())
    // transform.set_scale([scale.x, scale.y, scale.z].into());

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
        // If you want to check out generating messages using genmesh,
        // comment out the above lines, and uncomment the following:
        // let mesh = initialise_mesh(world);
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
        .with_dynamic_physical_entity(
            CollisionShape2::<f32, BodyPose2<f32>, ObjectType>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(scale[0]*2., scale[1]*2.).into(),
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
// This is another way to generate meshes, using
// the genmesh crate. It's faster and more efficient
// now to use the one in Amethyst.
#[allow(unused)]
fn initialise_mesh(world: &mut World) -> amethyst::assets::Handle<Mesh> {
    use genmesh::generators::Cube;
    use genmesh::Vertex;
    use genmesh::{MapToVertices, Triangulate, Vertices};
    use amethyst::renderer::rendy::mesh::PosTex;
    use amethyst::renderer::rendy::mesh::PosNormTangTex;
    use amethyst::assets::Loader;
    use amethyst::renderer::rendy::mesh::MeshBuilder;
    use amethyst::renderer::types::MeshData;

    let vertices:Vec<PosNormTangTex> = Cube::new()
        // .vertex(|v| PosTex {
        //     position: v.pos.into(),
        //     tex_coord: TexCoord::from([0.1, 0.1]),
        // })

        .vertex(|v| {
            info!("normal is {:?}", v.normal);
            // From amethyst_rendy/src/shape.rs:
            //   let tangent1 = normal.cross(&Vector3::x());
            //   let tangent2 = normal.cross(&Vector3::y());
            //   let tangent = if tangent1.norm_squared() > tangent2.norm_squared() {
            //       tangent1
            //   } else {
            //       tangent2
            //   }
            //   .cross(&normal);
            let n:amethyst::core::math::Vector3<f32> = v.normal.into();
            let tx = n.cross(&amethyst::core::math::Vector3::x());
            let ty = n.cross(&amethyst::core::math::Vector3::y());
            let t = if tx.norm_squared() > ty.norm_squared() {
                tx
            } else {
                ty
            };
            let tangent = t.cross(&n);
            //+ info!("got tangent {:?}", tangent);
            let p = tangent.data;
            let q = p.as_slice();
            let r:[f32;4] = [q[0], q[1], q[2], 1.];

            PosNormTangTex {
                position: v.pos.into(),
                normal: v.normal.into(),
                tangent: r.into(),
                tex_coord: TexCoord::from([0.1, 0.1]),
            }
        })
        .triangulate()
        .vertices()
        .collect::<Vec<_>>();
    // Mesh builder is expecting vectors of each element, not a combined vector
    // so split it out.
    let positions:Vec<Position> = vertices.iter().map(|v:&PosNormTangTex|->Position {
        v.position
    }).collect();
    let normals:Vec<Normal> = vertices.iter().map(|v:&PosNormTangTex|->Normal {
        v.normal
    }).collect();
    // Note: Tangents aren't necessary and don't need to be calculated.
    let tangents:Vec<Tangent> = vertices.iter().map(|v:&PosNormTangTex|->Tangent {
        v.tangent
    }).collect();
    let tex_coords:Vec<TexCoord> = vertices.iter().map(|v:&PosNormTangTex|->TexCoord {
        v.tex_coord
    }).collect();
    let mesh_builder = MeshBuilder::new()
        .with_vertices(positions)
        .with_vertices(normals)
        .with_vertices(tex_coords)
        .with_vertices(tangents)
    ;
    let mesh_data = MeshData::from(mesh_builder);
    world
        .read_resource::<Loader>()
        .load_from_data(mesh_data, (), &world.read_resource())
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
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 4.);
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
    add_rectangle(world, dimensions);
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
//                    Game State
//
// =================================================================================================

/// The game state: includes a reader to read the collision events.
#[derive(Default)]
struct GameState{
    reader: Option<ReaderId<ContactEvent2<f32>>>,
}

impl SimpleState for GameState {
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
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
            ),
            world,
        );
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
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
        .with_bundle(DefaultPhysicsBundle2::<ObjectType>::new().with_spatial())?
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

    let mut game = Application::new(
        resources,
        GameState::default(),
        game_data)?;
    game.run();

    Ok(())
}