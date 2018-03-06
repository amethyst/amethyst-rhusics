extern crate amethyst;
extern crate amethyst_rhusics;
extern crate collision;
extern crate genmesh;
extern crate rand;
extern crate rhusics_core;
extern crate rhusics_ecs;
extern crate specs;

#[path = "boxes/mod.rs"]
mod boxes;

use std::time::{Duration, Instant};

use amethyst::assets::{Handle, Loader};
use amethyst::core::{GlobalTransform, Transform, TransformBundle};
use amethyst::core::cgmath::{Array, Basis2, One, Point2, Quaternion, Vector3};
use amethyst::ecs::World;
use amethyst::prelude::{Application, Config, State, Trans};
use amethyst::renderer::{Camera, DisplayConfig, DrawFlat, Event, KeyboardInput, Material,
                         MaterialDefaults, Mesh, Pipeline, PosTex, RenderBundle, Stage,
                         VirtualKeyCode, WindowEvent};
use amethyst::utils::fps_counter::{FPSCounter, FPSCounterBundle};
use amethyst_rhusics::{time_sync, DefaultBasicPhysicsBundle2};
use collision::Aabb2;
use collision::primitive::{Primitive2, Rectangle};
use rhusics_core::{CollisionShape, RigidBody};
use rhusics_ecs::WithRigidBody;
use rhusics_ecs::physics2d::{BodyPose2, CollisionMode, CollisionStrategy, Mass2};

use self::boxes::{BoxSimulationBundle2, Emitter, Graphics, ObjectType};

pub struct Emitting;

pub type Shape = CollisionShape<Primitive2<f32>, BodyPose2<f32>, Aabb2<f32>, ObjectType>;

impl State for Emitting {
    fn on_start(&mut self, world: &mut World) {
        initialise_camera(world);
        let g = Graphics {
            mesh: initialise_mesh(world),
            material: initialise_material(world),
        };
        world.add_resource(g);
        initialise_walls(world);
        initialise_emitters(world);
    }

    fn update(&mut self, world: &mut World) -> Trans {
        time_sync(world);
        println!("FPS: {}", world.read_resource::<FPSCounter>().sampled_fps());
        Trans::None
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}

fn initialise_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::standard_2d())
        .with(Transform {
            rotation: Quaternion::one(),
            scale: Vector3::from_value(1.),
            translation: Vector3::new(0., 0., 5.),
        })
        .with(GlobalTransform::default())
        .build();
}

fn initialise_mesh(world: &mut World) -> Handle<Mesh> {
    use genmesh::{MapToVertices, Triangulate, Vertices};
    use genmesh::generators::Cube;
    let vertices = Cube::new()
        .vertex(|v| PosTex {
            position: v.pos.into(),
            tex_coord: [0.1, 0.1],
        })
        .triangulate()
        .vertices()
        .collect::<Vec<_>>();
    world
        .read_resource::<Loader>()
        .load_from_data(vertices.into(), (), &world.read_resource())
}

fn initialise_material(world: &mut World) -> Material {
    let albedo = world.read_resource::<Loader>().load_from_data(
        [0.7, 0.7, 0.7, 1.0].into(),
        (),
        &world.read_resource(),
    );
    Material {
        albedo,
        ..world.read_resource::<MaterialDefaults>().0.clone()
    }
}

fn initialise_walls(world: &mut World) {
    let (mesh, material) = {
        let g = world.read_resource::<Graphics>();

        let albedo = world.read_resource::<Loader>().load_from_data(
            [0.3, 0.3, 0.3, 1.0].into(),
            (),
            &world.read_resource(),
        );

        (
            g.mesh.clone(),
            Material {
                albedo,
                ..world.read_resource::<MaterialDefaults>().0.clone()
            },
        )
    };

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(GlobalTransform::default())
        .with(Transform {
            translation: Vector3::new(-0.9, 0., 0.),
            rotation: Quaternion::one(),
            scale: Vector3::new(0.1, 1.0, 1.0),
        })
        .with_static_rigid_body(
            Shape::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(0.2, 2.0).into(),
                ObjectType::Wall,
            ),
            BodyPose2::new(Point2::new(-0.9, 0.), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(GlobalTransform::default())
        .with(Transform {
            translation: Vector3::new(0.9, 0., 0.),
            rotation: Quaternion::one(),
            scale: Vector3::new(0.1, 1.0, 1.0),
        })
        .with_static_rigid_body(
            Shape::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(0.2, 2.0).into(),
                ObjectType::Wall,
            ),
            BodyPose2::new(Point2::new(0.9, 0.), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(GlobalTransform::default())
        .with(Transform {
            translation: Vector3::new(0., -0.9, 0.),
            rotation: Quaternion::one(),
            scale: Vector3::new(0.9, 0.1, 1.0),
        })
        .with_static_rigid_body(
            Shape::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(1.8, 0.2).into(),
                ObjectType::Wall,
            ),
            BodyPose2::new(Point2::new(0., -0.9), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(GlobalTransform::default())
        .with(Transform {
            translation: Vector3::new(0., 0.9, 0.),
            rotation: Quaternion::one(),
            scale: Vector3::new(0.9, 0.1, 1.0),
        })
        .with_static_rigid_body(
            Shape::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(1.8, 0.2).into(),
                ObjectType::Wall,
            ),
            BodyPose2::new(Point2::new(0., 0.9), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();
}

fn emitter(p: Point2<f32>, d: Duration) -> Emitter<Point2<f32>> {
    Emitter {
        location: p,
        emission_interval: d,
        last_emit: Instant::now(),
    }
}

fn initialise_emitters(world: &mut World) {
    world
        .create_entity()
        .with(emitter(
            Point2::new(-0.4, 0.),
            Duration::new(0, 500_000_000),
        ))
        .build();

    world
        .create_entity()
        .with(emitter(Point2::new(0.4, 0.), Duration::new(0, 750_000_000)))
        .build();

    world
        .create_entity()
        .with(emitter(Point2::new(0., -0.4), Duration::new(1, 0)))
        .build();

    world
        .create_entity()
        .with(emitter(Point2::new(0., 0.4), Duration::new(1, 250_000_000)))
        .build();
}

fn run() -> Result<(), amethyst::Error> {
    let path = format!(
        "{}/../resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let mut game = Application::build("./", Emitting)?
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(DefaultBasicPhysicsBundle2::<f32, ObjectType>::new())?
        .with_bundle(BoxSimulationBundle2::<f32>::new(
            Rectangle::new(0.1, 0.1).into(),
        ))?
        .with_bundle(TransformBundle::new().with_dep(&["sync_system"]))?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .build()
        .expect("Fatal error");

    game.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
