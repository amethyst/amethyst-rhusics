use std::marker::PhantomData;
use std::fmt::Debug;

use amethyst::renderer::{Query, Position, Color, ActiveCamera, Camera};
use amethyst::renderer::pipe::pass::{Pass, PassData};
use amethyst::core::cgmath::{VectorSpace, InnerSpace, Rotation, EuclideanSpace};
use amethyst::ecs::{Fetch, ReadStorage};
use rhusics::BodyPose;
use rhusics::collide::{Primitive, CollisionShape};

#[derive(Derivative, Clone, Debug, PartialEq)]
#[derivative(Default(bound = "V: Query<(Position, Color)>"))]
pub struct DrawPhysicsDebug<V, P, R, S> {
    _phantom: PhantomData<(V, P, R, S)>,
}

impl<V, P, R, S> DrawPhysicsDebug<V, P, R, S>
where
    V: Query<(Position, Color)>
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a, V, P, R, S> PassData<'a> for DrawPhysicsDebug<V, P, R, S>
where
    V: Query<(Position, Color)>,
    P: EuclideanSpace<Scalar = f32> + Send + Sync + 'static,
    P::Diff: VectorSpace<Scalar = f32> + InnerSpace + Debug + Send + Sync + 'static,
    R: Rotation<P> + Send + Sync + 'static,
    S: Primitive + Send + Sync + 'static,
    S::Aabb: Clone
        + Debug
        + Send
        + Sync
        + 'static
{
    type Data = (
        Option<Fetch<'a, ActiveCamera>>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, BodyPose<P, R>>,
        ReadStorage<'a, CollisionShape<S, BodyPose<P, R>, ()>>,
    );
}
