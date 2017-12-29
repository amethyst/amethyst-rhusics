use std::time::Instant;

use amethyst::assets::Handle;
use amethyst::core::{LocalTransform, Transform};
use amethyst::core::cgmath::{Array, One, Point2, Quaternion, Vector2, Vector3, Zero, InnerSpace,
                             EuclideanSpace};
use amethyst::ecs::{Entities, Entity, Fetch, Join, LazyUpdate, System, WriteStorage};
use amethyst::renderer::{Material, Mesh};

use rhusics::ecs::collide::prelude2d::*;
use rhusics::collide::prelude2d::BodyPose2;
use rhusics::physics::prelude2d::{Mass2, Velocity2};
use rhusics::physics::Material as PhysicsMaterial;
use rhusics::ecs::physics::prelude2d::RigidBody;
use rhusics::ecs::physics::WithLazyRigidBody;

use resources::{Emitter, Graphics};

pub struct EmissionSystem;

/// Handles emitter components that are responsible for projecting
/// 2D, physically simulated box entities into the world to demonstrate
/// interaction between rigid bodies.
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
    use rand::Rand;
    use rand::Rng;
    use std;

    let target: Vector2<f32> = Vector2::rand(&mut rand::thread_rng()).normalize() * 0.5;
    let speed = rand::thread_rng().gen_range(1., 2.);
    let position = Point2::new(emitter.location.0, emitter.location.1);

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

    let volume = Rectangle::new(0.1, 0.1);

    lazy.with_dynamic_rigid_body(
        entity,
        CollisionShape2::<BodyPose2, ()>::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            volume.clone().into(),
        ),
        BodyPose2::new(position, Basis2::one()),
        Velocity2::new(
            (target - position.to_vec()).normalize() * speed,
            0.
        ),
        RigidBody::new(PhysicsMaterial::ROCK, 1.),
        Mass2::from_volume_and_material(&volume, &PhysicsMaterial::ROCK),
    );
}
