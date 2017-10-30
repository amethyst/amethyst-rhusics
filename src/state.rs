use std::time::{Duration, Instant};

use amethyst::prelude::{Engine, State, Trans};
use amethyst::assets::{Handle, Loader};
use amethyst::ecs::World;
use amethyst::core::{LocalTransform, Transform};
use amethyst::renderer::{Camera, Event, KeyboardInput, Material, MaterialDefaults, Mesh, PosTex,
                         VirtualKeyCode, WindowEvent};
use amethyst::core::cgmath::{Array, One, Point3, Quaternion, Vector3};
use amethyst::utils::fps_counter::FPSCounter;
use rhusics::ecs::collide::prelude3d::{BodyPose3, CollisionMode, CollisionStrategy, Cuboid};

use resources::{Emitter, Graphics, ObjectType, Shape};

pub struct Emitting;

impl State for Emitting {
    fn on_start(&mut self, engine: &mut Engine) {
        initialise_camera(&mut engine.world);
        let g = Graphics {
            mesh: initialise_mesh(&mut engine.world),
            material: initialise_material(&mut engine.world),
        };
        engine.world.add_resource(g);
        initialise_walls(&mut engine.world);
        initialise_emitters(&mut engine.world);
    }

    fn update(&mut self, engine: &mut Engine) -> Trans {
        println!("FPS: {}", engine.world.read_resource::<FPSCounter>().sampled_fps());
        Trans::None
    }

    fn handle_event(&mut self, _: &mut Engine, event: Event) -> Trans {
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
    use genmesh::generators::Cube;
    use genmesh::{MapToVertices, Triangulate, Vertices};
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
        .with(BodyPose3::new(Point3::new(-0.9, 0., 0.), Quaternion::one()))
        .with(Shape::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Cuboid::new(0.2, 2.0, 2.0).into(),
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
        .with(BodyPose3::new(Point3::new(0.9, 0., 0.), Quaternion::one()))
        .with(Shape::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Cuboid::new(0.2, 2.0, 2.0).into(),
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
        .with(BodyPose3::new(Point3::new(0., -0.9, 0.), Quaternion::one()))
        .with(Shape::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Cuboid::new(1.8, 0.2, 2.0).into(),
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
        .with(BodyPose3::new(Point3::new(0., 0.9, 0.), Quaternion::one()))
        .with(Shape::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Cuboid::new(1.8, 0.2, 2.0).into(),
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
