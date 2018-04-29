extern crate amethyst;
extern crate amethyst_rhusics;
extern crate collision;
extern crate genmesh;
extern crate rand;
extern crate rhusics_core;
extern crate rhusics_ecs;
extern crate shred;
#[macro_use]
extern crate shred_derive;
extern crate specs;

use std::time::{Duration, Instant};

use amethyst::assets::{Handle, Loader};
use amethyst::core::{GlobalTransform, Transform, TransformBundle};
use amethyst::core::cgmath::{Array, One, Point3, Quaternion, Vector3};
use amethyst::ecs::prelude::World;
use amethyst::prelude::{Application, Config, State, Trans};
use amethyst::renderer::{Camera, DisplayConfig, DrawFlat, Event, KeyboardInput, Material,
                         MaterialDefaults, Mesh, Pipeline, PosTex, RenderBundle, Stage,
                         VirtualKeyCode, WindowEvent};
use amethyst::utils::fps_counter::{FPSCounter, FPSCounterBundle};
use amethyst_rhusics::{time_sync, DefaultPhysicsBundle3, setup_3d_arena};
use collision::Aabb3;
use collision::primitive::{Cuboid, Primitive3};
use rhusics_core::CollisionShape;
use rhusics_ecs::physics3d::BodyPose3;

use self::boxes::{BoxSimulationBundle3, Emitter, Graphics, ObjectType};

mod boxes;

pub struct Emitting;

pub type Shape = CollisionShape<Primitive3<f32>, BodyPose3<f32>, Aabb3<f32>, ObjectType>;

impl State for Emitting {
    fn on_start(&mut self, world: &mut World) {
        initialise_camera(world);
        let g = Graphics {
            mesh: initialise_mesh(world),
            material: initialise_material(world),
        };
        world.add_resource(g);
        setup_3d_arena(
            Point3::new(-1., -1., -2.),
            Point3::new(1., 1., 0.),
            (
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
            ),
            world,
        );
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
        .with(Camera::standard_3d(500., 500.))
        .with(Transform {
            rotation: Quaternion::one(),
            scale: Vector3::from_value(1.),
            translation: Vector3::new(0., 0., 1.),
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

fn emitter(p: Point3<f32>, d: Duration) -> Emitter<Point3<f32>> {
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
            Point3::new(-0.4, 0., -1.),
            Duration::new(0, 500_000_000),
        ))
        .build();

    world
        .create_entity()
        .with(emitter(
            Point3::new(0.4, 0., -1.),
            Duration::new(0, 750_000_000),
        ))
        .build();

    world
        .create_entity()
        .with(emitter(Point3::new(0., -0.4, -1.), Duration::new(1, 0)))
        .build();

    world
        .create_entity()
        .with(emitter(
            Point3::new(0., 0.4, -1.),
            Duration::new(1, 250_000_000),
        ))
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
        .with_bundle(DefaultPhysicsBundle3::<ObjectType>::new().with_spatial())?
        .with_bundle(BoxSimulationBundle3::new(Cuboid::new(0.1, 0.1, 0.1).into()))?
        .with_bundle(TransformBundle::new().with_dep(&["sync_system", "emission_system"]))?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .build()?;

    game.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
