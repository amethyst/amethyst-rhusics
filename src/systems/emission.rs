use std::time::Instant;

use amethyst::assets::Handle;
use amethyst::ecs::{Entities, Entity, Fetch, Join, LazyUpdate, System, WriteStorage};
use amethyst::core::cgmath::{Array, EuclideanSpace, One, Point3, Quaternion, Vector3};
use amethyst::core::{LocalTransform, Transform};
use amethyst::renderer::{Material, Mesh};
use rhusics::ecs::collide::prelude3d::{BodyPose3, CollisionMode, CollisionStrategy, Cuboid};
use rhusics::NextFrame;

use resources::{Emitter, Graphics, ObjectType, Shape, Velocity};

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
                    graphics.mesh.clone(),
                    graphics.material.clone(),
                    &emitter,
                );
                emitter.last_emit = now.clone();
            }
        }
    }
}

fn emit_box(
    entity: Entity,
    lazy: &LazyUpdate,
    mesh: Handle<Mesh>,
    material: Material,
    emitter: &Emitter,
) {
    use std;
    use rand;
    use rand::Rng;
    use amethyst::core::cgmath::{Rotation3, Rad, Rotation};

    let angle = rand::thread_rng().gen_range(0., std::f32::consts::PI * 2.);
    let rot = Quaternion::from_angle_z(Rad(angle));
    let offset = rot.rotate_vector(Vector3::new(0.1, 0., 0.));
    let speed = rand::thread_rng().gen_range(0.,5.) * 2.;

    // TODO: offset position
    // TODO: randomize velocity
    let position = Point3::new(emitter.location.0, emitter.location.1, 0.) + offset;
    lazy.insert(entity, ObjectType::Box);
    lazy.insert(entity, mesh);
    lazy.insert(entity, material);
    lazy.insert(
        entity,
        Velocity {
            linear: offset * speed,
        },
    );
    lazy.insert(entity, Transform::default());
    lazy.insert(
        entity,
        LocalTransform {
            translation: position.to_vec(),
            rotation: Quaternion::one(),
            scale: Vector3::from_value(0.05),
        },
    );
    let pose = BodyPose3::new(position, Quaternion::one());
    lazy.insert(entity, pose.clone());
    lazy.insert(entity, NextFrame { value: pose });
    lazy.insert(
        entity,
        Shape::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Cuboid::new(0.1, 0.1, 0.1).into(),
        ),
    );
}
