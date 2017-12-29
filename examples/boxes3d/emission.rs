use std::time::Instant;
use rand::Rand;

use amethyst::assets::Handle;
use amethyst::core::{LocalTransform, Transform};
use amethyst::core::cgmath::{Array, One, Point3, Quaternion, Vector3, InnerSpace, Zero, EuclideanSpace};
use amethyst::ecs::{Entities, Entity, Fetch, Join, LazyUpdate, System, WriteStorage};
use amethyst::renderer::{Material, Mesh};
use rhusics::ecs::collide::prelude3d::*;
use rhusics::collide::prelude3d::BodyPose3;
use rhusics::NextFrame;
use rhusics::physics::prelude3d::{Mass3, Velocity3};
use rhusics::physics::Material as PhysicsMaterial;
use rhusics::ecs::physics::prelude3d::RigidBody;
use rhusics::ecs::physics::WithLazyRigidBody;

use resources::{Emitter, Graphics, Shape};

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

    // Determine a target location within 0.5 units of the origin.
    let target: Vector3<f32> = Vector3::rand(&mut rand::thread_rng()).normalize() * 0.5;
    // Determine a randomized speed.
    let speed = rand::thread_rng().gen_range(1., 2.) * 2.;
    let position = Point3::new(emitter.location.0, emitter.location.1, emitter.location.2);

    lazy.insert(entity, mesh);
    lazy.insert(entity, material);
    lazy.insert(entity,Transform::default());
    lazy.insert(
        entity,
        LocalTransform {
            translation: Vector3::new(position.x, position.y, position.z),
            rotation: Quaternion::one(),
            scale: Vector3::from_value(1.),
        },
    );

    let volume = Cuboid::new(2.25, 2.25, 2.25);

    lazy.with_dynamic_rigid_body(
        entity,
        CollisionShape3::<BodyPose3, ()>::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            volume.clone().into(),
        ),
        BodyPose3::new(position, Quaternion::one()),
        Velocity3::new(
            (target - position.to_vec()).normalize() * speed,
            Vector3::zero(),
        ),
        RigidBody::new(PhysicsMaterial::ROCK, 1.),
        Mass3::from_volume_and_material(&volume, &PhysicsMaterial::ROCK),
    );
}
