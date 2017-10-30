use std::time::Instant;

use amethyst::assets::Handle;
use amethyst::ecs::{Entities, Entity, Fetch, Join, LazyUpdate, System, WriteStorage};
use amethyst::core::cgmath::{Array, One, Quaternion, Vector3, Vector2, Point2};
use amethyst::core::{LocalTransform, Transform};
use amethyst::renderer::{Material, Mesh};
use rhusics::ecs::collide::prelude2d::{BodyPose2, CollisionMode, CollisionStrategy, Rectangle};
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
    use amethyst::core::cgmath::{Rad, Rotation, Rotation2, Basis2};

    let angle = rand::thread_rng().gen_range(0., std::f32::consts::PI * 2.);
    let rot : Basis2<f32> = Rotation2::from_angle(Rad(angle));
    let offset = rot.rotate_vector(Vector2::new(0.1, 0.));
    let speed = rand::thread_rng().gen_range(0.,5.) * 2.;

    // TODO: offset position
    // TODO: randomize velocity
    let position = Point2::new(emitter.location.0, emitter.location.1) + offset;
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
            translation: Vector3::new(position.x, position.y, 0.),
            rotation: Quaternion::one(),
            scale: Vector3::from_value(0.05),
        },
    );
    let pose = BodyPose2::new(position, Basis2::one());
    lazy.insert(entity, pose.clone());
    lazy.insert(entity, NextFrame { value: pose });
    lazy.insert(
        entity,
        Shape::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Rectangle::new(0.1, 0.1).into(),
        ),
    );
}
