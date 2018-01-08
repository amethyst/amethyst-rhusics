use std::time::{Duration, Instant};

use amethyst::assets::Handle;
use amethyst::ecs::{Component, DenseVecStorage, VecStorage};
use amethyst::renderer::{Material, Mesh};
use rhusics_core::Collider;
use rhusics_ecs::collide2d::{BodyPose2, CollisionShape2};

pub type Shape = CollisionShape2<f32, BodyPose2<f32>, ObjectType>;

#[repr(u8)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ObjectType {
    Wall,
    Box,
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Box
    }
}

impl Collider for ObjectType {
    fn should_generate_contacts(&self, other: &ObjectType) -> bool {
        self != other || *self != ObjectType::Wall
    }
}

impl Component for ObjectType {
    type Storage = VecStorage<Self>;
}

pub struct Emitter {
    pub location: (f32, f32),
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
