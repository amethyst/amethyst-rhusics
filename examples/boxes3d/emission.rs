use std::time::Instant;
use rand::Rand;

use amethyst::assets::Handle;
use amethyst::core::{LocalTransform, Transform};
use amethyst::core::cgmath::{Array, One, Point3, Quaternion, Vector3, InnerSpace};
use amethyst::ecs::{Entities, Entity, Fetch, Join, LazyUpdate, System, WriteStorage};
use amethyst::renderer::{Material, Mesh};
use rhusics::ecs::collide::prelude3d::*;
use rhusics::collide::prelude3d::BodyPose3;
use rhusics::NextFrame;
use rhusics::physics::prelude3d::{Mass3, Velocity3};
use rhusics::physics::Material as PhysicsMaterial;
use rhusics::ecs::physics::prelude3d::RigidBody;

use resources::{Emitter, Graphics, Shape, ObjectType};

pub struct EmissionSystem;

impl<'a> System<'a> for EmissionSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, LazyUpdate>,
        Fetch<'a, Graphics>,
        WriteStorage<'a, Emitter>,
    );

    fn run(&mut self, (entities, lazy, graphics, mut emitters): Self::SystemData) {
        let now = Instant::now();
        for emitter in (&mut emitters).join() {
            if (now - emitter.last_emit) > emitter.emission_interval {
                emit_box(
                    entities.create(),
                    &*lazy,
                    graphics.material.clone(),
                    graphics.mesh.clone(),
                    &emitter,
                );
                emitter.last_emit = now.clone();
            }
        }
    }
}

fn emit_box(entity: Entity, lazy: &LazyUpdate, material: Material, mesh: Handle<Mesh>, emitter: &Emitter) {
    use amethyst::core::cgmath::{Basis3, Rad, Rotation, Rotation3, Vector3};
    use rand;
    use rand::Rng;
    use std;

    let rot: Basis3<f32> = Rotation3::from_axis_angle(
        Vector3::rand(&mut rand::thread_rng()).normalize(),
        Rad(rand::thread_rng().gen_range(0., std::f32::consts::PI * 2.)),
    );
    let offset = rot.rotate_vector(Vector3::new(0.1, 0., 0.));
    let speed = rand::thread_rng().gen_range(1., 5.) * 2.;


    let position = Point3::new(emitter.location.0, emitter.location.1, emitter.location.2) + offset;
    println!("pos: {:?}", position);
    lazy.insert(entity, ObjectType::Box);
    lazy.insert(entity, mesh);
    lazy.insert(entity, material);
    lazy.insert(
        entity,
        Velocity3::new(offset * speed, Vector3::from_value(0.)),
    );
    lazy.insert(
        entity,
        LocalTransform {
            translation: Vector3::new(position.x, position.y, position.z),
            rotation: Quaternion::one(),
            scale: Vector3::from_value(1.),
        },
    );

    let pose = BodyPose3::new(position, Quaternion::one());
    lazy.insert(entity, pose.clone());
    lazy.insert(entity, NextFrame { value: pose });
    lazy.insert(
        entity,
        NextFrame {
            value: Velocity3::new(offset * speed, Vector3::from_value(0.1)),
        },
    );
    lazy.insert(entity, Mass3::new(1.));
    lazy.insert(entity, RigidBody::new(PhysicsMaterial::ROCK, 1.0));
    lazy.insert(
        entity,
        Shape::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Cuboid::new(0.1, 0.1, 0.1).into(),
        ),
    );
}
