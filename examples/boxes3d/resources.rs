use std::time::{Duration, Instant};

use amethyst::assets::Handle;
use amethyst::ecs::{Component, DenseVecStorage, VecStorage};
use amethyst::renderer::{Material, Mesh};
use rhusics::ecs::collide::prelude3d::{BodyPose3, CollisionShape3};

pub type Shape = CollisionShape3<BodyPose3>;

pub struct Emitter {
    pub location: (f32, f32, f32),
    pub last_emit: Instant,
    pub emission_interval: Duration,
}

impl Component for Emitter {
    type Storage = DenseVecStorage<Self>;
}

pub struct Graphics {
    pub mesh: Handle<Mesh>,
    pub material: Material,
}
