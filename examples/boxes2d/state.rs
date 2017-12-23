use std::time::{Duration, Instant};

use amethyst::assets::{Handle, Loader};
use amethyst::core::{LocalTransform, Time, Transform};
use amethyst::core::cgmath::{Array, One, Quaternion, Vector3};
use amethyst::ecs::World;
use amethyst::prelude::{State, Trans};
use amethyst::renderer::{Camera, Event, KeyboardInput, Material, MaterialDefaults, Mesh, PosTex,
                         VirtualKeyCode, WindowEvent};
use amethyst::utils::fps_counter::FPSCounter;
use rhusics::ecs::physics::prelude2d::DeltaTime;

use resources::{Emitter, Graphics};

pub struct Emitting;

impl State for Emitting {
    fn on_start(&mut self, world: &mut World) {
        initialise_camera(world);

        let g = Graphics {
            mesh: initialise_mesh(world),
            material: initialise_material(world),
        };

        world.add_resource(g);
        initialise_emitters(world);
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let mut delta = world.write_resource::<DeltaTime>();
        let time = world.read_resource::<Time>();
        delta.delta_seconds = time.delta_seconds();
        println!(
            "FPS: {}",
            world.read_resource::<FPSCounter>().sampled_fps()
        );
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
        .with(LocalTransform {
            rotation: Quaternion::one(),
            scale: Vector3::from_value(1.),
            translation: Vector3::new(0., 0., 5.),
        })
        .with(Transform::default())
        .build();
}

fn initialise_mesh(world: &mut World) -> Handle<Mesh> {
    use genmesh::{MapToVertices, Triangulate, Vertices};
    use genmesh::generators::Cube;
    let vertices = Cube::new()
        .vertex(|v| {
            PosTex {
                position: v.pos.into(),
                tex_coord: [0.1, 0.1],
            }
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

fn initialise_emitters(world: &mut World) {
    world
        .create_entity()
        .with(Emitter {
            location: (-0.4, 0.),
            emission_interval: Duration::new(0, 500_000_000),
            last_emit: Instant::now(),
        })
        .build();

    world
        .create_entity()
        .with(Emitter {
            location: (0.4, 0.),
            emission_interval: Duration::new(0, 750_000_000),
            last_emit: Instant::now(),
        })
        .build();

    world
        .create_entity()
        .with(Emitter {
            location: (0., -0.4),
            emission_interval: Duration::new(1, 0),
            last_emit: Instant::now(),
        })
        .build();

    world
        .create_entity()
        .with(Emitter {
            location: (0., 0.4),
            emission_interval: Duration::new(1, 250_000_000),
            last_emit: Instant::now(),
        })
        .build();
}
