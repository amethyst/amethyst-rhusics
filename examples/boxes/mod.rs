pub use self::bundle::BoxSimulationBundle;
pub use self::deletion::BoxDeletionSystem;
pub use self::emission::EmissionSystem;

mod deletion;
mod emission;
mod bundle;

use std::time::{Duration, Instant};

use amethyst::assets::Handle;
use amethyst::ecs::{Component, DenseVecStorage, VecStorage};
use amethyst::renderer::{Material, Mesh};
use rhusics_core::Collider;

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

pub struct Emitter<P> {
    pub location: P,
    pub last_emit: Instant,
    pub emission_interval: Duration,
}

impl<P> Component for Emitter<P>
where
    P: Send + Sync + 'static,
{
    type Storage = DenseVecStorage<Self>;
}

pub struct Graphics {
    pub mesh: Handle<Mesh>,
    pub material: Material,
}
