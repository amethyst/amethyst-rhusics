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

use std::time::{Duration, Instant};

use amethyst::assets::{Handle, Loader};
use amethyst::core::{GlobalTransform, Transform, TransformBundle};
use amethyst::core::cgmath::{Array, One, Point2, Quaternion, Vector3};
use amethyst::ecs::prelude::{Entity, World};
use amethyst::prelude::{Application, Config, State, Trans};
use amethyst::renderer::{Camera, DisplayConfig, DrawFlat, Event, KeyboardInput, Material,
                         MaterialDefaults, Mesh, Pipeline, PosTex, RenderBundle, Stage,
                         VirtualKeyCode, WindowEvent};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::fps_counter::FPSCounterBundle;
use amethyst_rhusics::{time_sync, DefaultPhysicsBundle2, setup_2d_arena};
use collision::Aabb2;
use collision::primitive::{Primitive2, Rectangle};
use rhusics_core::CollisionShape;
use rhusics_ecs::physics2d::BodyPose2;

use self::boxes::{create_ui, update_ui, BoxSimulationBundle2, Emitter, Graphics, KillRate,
                  ObjectType};

mod boxes;

#[derive(Default)]
pub struct Emitting {
    fps: Option<Entity>,
    num: Option<Entity>,
    collision: Option<Entity>,
}

pub type Shape = CollisionShape<Primitive2<f32>, BodyPose2<f32>, Aabb2<f32>, ObjectType>;

impl State for Emitting {
    fn on_start(&mut self, world: &mut World) {
        world.write_resource::<KillRate>().0 = 0.01;
        initialise_camera(world);
        let g = Graphics {
            mesh: initialise_mesh(world),
        };
        let (num_display, fps_display, collisions_display) = create_ui(world);
        self.num = Some(num_display);
        self.fps = Some(fps_display);
        self.collision = Some(collisions_display);

        world.add_resource(g);
        setup_2d_arena(
            Point2::new(-1., -1.),
            Point2::new(1., 1.),
            (
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
            ),
            world,
        );
        initialise_emitters(world);
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

    fn update(&mut self, world: &mut World) -> Trans {
        time_sync(world);
        update_ui::<Point2<f32>>(
            world,
            self.num.unwrap(),
            self.fps.unwrap(),
            self.collision.unwrap(),
        );
        Trans::None
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

fn initialise_material(world: &mut World, r: f32, g: f32, b: f32) -> Material {
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

fn emitter(p: Point2<f32>, d: Duration, material: Material) -> Emitter<Point2<f32>> {
    Emitter {
        location: p,
        emission_interval: d,
        last_emit: Instant::now(),
        material,
        emitted: 0,
    }
}

fn initialise_emitters(world: &mut World) {
    let mat = initialise_material(world, 0.3, 1.0, 0.3);
    world
        .create_entity()
        .with(emitter(
            Point2::new(-0.4, 0.),
            Duration::new(0, 50_000_000),
            mat,
        ))
        .build();

    let mat = initialise_material(world, 0.3, 0.0, 0.3);
    world
        .create_entity()
        .with(emitter(
            Point2::new(0.4, 0.),
            Duration::new(0, 75_000_000),
            mat,
        ))
        .build();

    let mat = initialise_material(world, 1.0, 1.0, 1.0);
    world
        .create_entity()
        .with(emitter(
            Point2::new(0., -0.4),
            Duration::new(0, 100_000_000),
            mat,
        ))
        .build();

    let mat = initialise_material(world, 1.0, 0.3, 0.3);
    world
        .create_entity()
        .with(emitter(
            Point2::new(0., 0.4),
            Duration::new(1, 25_000_000),
            mat,
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
            .with_pass(DrawFlat::<PosTex>::new())
            .with_pass(DrawUi::new()),
    );

    let mut game = Application::build("./", Emitting::default())?
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(DefaultPhysicsBundle2::<ObjectType>::new().with_spatial())?
        .with_bundle(BoxSimulationBundle2::new(Rectangle::new(0.1, 0.1).into()))?
        .with_bundle(TransformBundle::new().with_dep(&["sync_system", "emission_system"]))?
        .with_bundle(UiBundle::<String, String>::new())?
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
