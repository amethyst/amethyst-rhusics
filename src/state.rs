use std::time::{Duration, Instant};

use amethyst::assets::{Handle, Loader};
use amethyst::core::{LocalTransform, Time, Transform};
use amethyst::core::cgmath::{Array, Basis2, One, Point2, Quaternion, Vector3};
use amethyst::ecs::World;
use amethyst::prelude::{State, Trans};
use amethyst::renderer::{Camera, Event, KeyboardInput, Material, MaterialDefaults, Mesh, PosTex,
                         VirtualKeyCode, WindowEvent};
use amethyst::utils::fps_counter::FPSCounter;
use rhusics::ecs::physics::prelude2d::{BodyPose2, CollisionMode, CollisionStrategy, DeltaTime,
                                       Rectangle};

use resources::{Emitter, Graphics, ObjectType, Shape};

pub struct Emitting;

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
        .with(Transform::default())
        .with(LocalTransform {
            translation: Vector3::new(-0.9, 0., 0.),
            rotation: Quaternion::one(),
            scale: Vector3::new(0.1, 1.0, 1.0),
        })
        .with(ObjectType::Wall)
        .with(BodyPose2::new(Point2::new(-0.9, 0.), Basis2::one()))
        .with(Shape::new_simple_with_type(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Rectangle::new(0.2, 2.0).into(),
            ObjectType::Wall,
        ))
        .build();

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(Transform::default())
        .with(LocalTransform {
            translation: Vector3::new(0.9, 0., 0.),
            rotation: Quaternion::one(),
            scale: Vector3::new(0.1, 1.0, 1.0),
        })
        .with(ObjectType::Wall)
        .with(BodyPose2::new(Point2::new(0.9, 0.), Basis2::one()))
        .with(Shape::new_simple_with_type(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Rectangle::new(0.2, 2.0).into(),
            ObjectType::Wall,
        ))
        .build();

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(Transform::default())
        .with(LocalTransform {
            translation: Vector3::new(0., -0.9, 0.),
            rotation: Quaternion::one(),
            scale: Vector3::new(0.9, 0.1, 1.0),
        })
        .with(ObjectType::Wall)
        .with(BodyPose2::new(Point2::new(0., -0.9), Basis2::one()))
        .with(Shape::new_simple_with_type(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Rectangle::new(1.8, 0.2).into(),
            ObjectType::Wall,
        ))
        .build();

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(Transform::default())
        .with(LocalTransform {
            translation: Vector3::new(0., 0.9, 0.),
            rotation: Quaternion::one(),
            scale: Vector3::new(0.9, 0.1, 1.0),
        })
        .with(ObjectType::Wall)
        .with(BodyPose2::new(Point2::new(0., 0.9), Basis2::one()))
        .with(Shape::new_simple_with_type(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Rectangle::new(1.8, 0.2).into(),
            ObjectType::Wall,
        ))
        .build();
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
