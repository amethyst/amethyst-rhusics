use std::marker;
use std::time::Instant;

use amethyst::assets::Handle;
use amethyst::core::GlobalTransform;
use amethyst::core::cgmath::{Array, EuclideanSpace, InnerSpace, Quaternion,
                             Rotation, Vector3, Zero};
use amethyst::ecs::{Entities, Entity, Fetch, Join, LazyUpdate, System, WriteStorage};
use amethyst::renderer::{Material, Mesh};
use amethyst_rhusics::{AsTransform, Convert};
use collision::{Bound, ComputeBound, Primitive, Union};
use rand::Rand;
use rhusics_core::{BodyPose, CollisionMode, CollisionShape, CollisionStrategy, Inertia, Mass,
                   RigidBody, Velocity};
use rhusics_ecs::WithLazyRigidBody;

use super::{Emitter, Graphics, ObjectType};

/// Primitive emission system.
///
/// Will spawn new primitives regularly, based on the `Emitter`s in the `World`
///
/// ### Type parameters:
///
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (usually `Aabb2`, `Aabb3` or `Sphere`)
/// - `R`: Rotational quantity (`Basis2` or `Quaternion`)
/// - `A`: Angular velocity quantity (`Scalar` or `Vector3`)
/// - `I`: Inertia tensor (`Scalar` or `Matrix3`)
pub struct EmissionSystem<P, B, R, A, I> {
    primitive: P,
    m: marker::PhantomData<(B, R, A, I)>,
}

impl<P, B, R, A, I> EmissionSystem<P, B, R, A, I> {
    pub fn new(primitive: P) -> Self {
        Self {
            primitive,
            m: marker::PhantomData,
        }
    }
}

impl<'a, P, B, R, A, I> System<'a> for EmissionSystem<P, B, R, A, I>
where
    B: Bound<Point = P::Point> + Union<B, Output = B> + Clone + Send + Sync + 'static,
    P: Primitive + ComputeBound<B> + Clone + Send + Sync + 'static,
    P::Point: EuclideanSpace<Scalar = f32> + Convert<Output = Vector3<f32>> + Send + Sync + 'static,
    <P::Point as EuclideanSpace>::Diff: Rand + InnerSpace + Array + Send + Sync + 'static,
    R: Rotation<P::Point> + Convert<Output = Quaternion<f32>> + Send + Sync + 'static,
    A: Clone + Copy + Zero + Send + Sync + 'static,
    I: Inertia + Send + Sync + 'static,
{
    type SystemData = (
        Entities<'a>,
        Fetch<'a, LazyUpdate>,
        Fetch<'a, Graphics>,
        WriteStorage<'a, Emitter<P::Point>>,
    );

    fn run(&mut self, (entities, lazy, graphics, mut emitters): Self::SystemData) {
        let now = Instant::now();
        for emitter in (&mut emitters).join() {
            if (now - emitter.last_emit) > emitter.emission_interval {
                emit_box::<P, B, R, A, I>(
                    entities.create(),
                    &*lazy,
                    graphics.mesh.clone(),
                    graphics.material.clone(),
                    &emitter,
                    &self.primitive,
                );
                emitter.last_emit = now.clone();
            }
        }
    }
}

fn emit_box<P, B, R, A, I>(
    entity: Entity,
    lazy: &LazyUpdate,
    mesh: Handle<Mesh>,
    material: Material,
    emitter: &Emitter<P::Point>,
    primitive: &P,
) where
    B: Bound<Point = P::Point> + Union<B, Output = B> + Clone + Send + Sync + 'static,
    P: Primitive + ComputeBound<B> + Clone + Send + Sync + 'static,
    P::Point: EuclideanSpace<Scalar = f32> + Convert<Output = Vector3<f32>> + Send + Sync + 'static,
    <P::Point as EuclideanSpace>::Diff: Rand + InnerSpace + Array + Send + Sync + 'static,
    R: Rotation<P::Point> + Convert<Output = Quaternion<f32>> + Send + Sync + 'static,
    A: Clone + Copy + Zero + Send + Sync + 'static,
    I: Inertia + Send + Sync + 'static,
{
    use rand;
    use rand::Rng;

    let offset = <P::Point as EuclideanSpace>::Diff::rand(&mut rand::thread_rng())
        .normalize_to(0.1);
    let speed: <P::Point as EuclideanSpace>::Scalar =
        rand::thread_rng().gen_range(-10.0, 10.0);

    let position = emitter.location + offset;
    let pose = BodyPose::new(position, R::one());
    let mut transform = pose.as_transform();
    transform.scale = Vector3::from_value(0.05);

    lazy.insert(entity, ObjectType::Box);
    lazy.insert(entity, mesh);
    lazy.insert(entity, material);
    lazy.insert(entity, GlobalTransform::default());

    lazy.with_dynamic_rigid_body(
        entity,
        CollisionShape::<P, BodyPose<P::Point, R>, B, ObjectType>::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            primitive.clone(),
        ),
        pose,
        Velocity::<<P::Point as EuclideanSpace>::Diff, A>::from_linear(offset * speed),
        RigidBody::default(),
        Mass::<f32, I>::new(
            1.,
        ),
    );
    lazy.insert(entity, transform);
}
