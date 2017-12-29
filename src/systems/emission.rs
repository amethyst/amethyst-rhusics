use std::time::Instant;

use amethyst::assets::Handle;
use amethyst::core::{LocalTransform, Transform};
use amethyst::core::cgmath::{Array, One, Point2, Quaternion, Vector2, Vector3};
use amethyst::ecs::{Entities, Entity, Fetch, Join, LazyUpdate, System, WriteStorage};
use amethyst::renderer::{Material, Mesh};
use rhusics::ecs::physics::prelude2d::{BodyPose2, CollisionMode, CollisionStrategy, Mass2,
                                       Rectangle, RigidBody, Velocity2, WithLazyRigidBody};

use resources::{Emitter, Graphics, ObjectType, Shape};

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
    use amethyst::core::cgmath::{Basis2, Rad, Rotation, Rotation2};
    use rand;
    use rand::Rng;
    use std;

    let angle = rand::thread_rng().gen_range(0., std::f32::consts::PI * 2.);
    let rot: Basis2<f32> = Rotation2::from_angle(Rad(angle));
    let offset = rot.rotate_vector(Vector2::new(0.1, 0.));
    let speed = rand::thread_rng().gen_range(1., 5.) * 2.;

    let position = Point2::new(emitter.location.0, emitter.location.1) + offset;
    lazy.with_dynamic_rigid_body(
        entity,
        Shape::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Rectangle::new(0.1, 0.1).into(),
        ),
        BodyPose2::new(position, Basis2::one()),
        Velocity2::from_linear(offset * speed),
        RigidBody::default(),
        Mass2::new(1.),
    );
    lazy.insert(entity, ObjectType::Box);
    lazy.insert(entity, mesh);
    lazy.insert(entity, material);
    lazy.insert(entity, Transform::default());
    lazy.insert(
        entity,
        LocalTransform {
            translation: Vector3::new(position.x, position.y, 0.),
            rotation: Quaternion::one(),
            scale: Vector3::from_value(0.05),
        },
    );
}
