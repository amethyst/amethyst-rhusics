pub use self::bundle::BoxSimulationBundle;
pub use self::default::{BoxSimulationBundle2, BoxSimulationBundle3};
pub use self::deletion::BoxDeletionSystem;
pub use self::emission::EmissionSystem;
pub use self::ui::*;

mod bundle;
mod default;
mod deletion;
mod emission;
mod ui;

use std::time::{Duration, Instant};

use amethyst::assets::Handle;
use amethyst::ecs::prelude::{Component, DenseVecStorage, VecStorage};
use amethyst::renderer::{Material, Mesh};
use rhusics_core::Collider;

/// Collision object type.
///
/// Used by collision detection to determine if shapes should be checked for collisions or not.
/// Only box-box collisions will be processed.
///
/// Showcase how `Collider` works.
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
    /// Remove collision testing for Wall - Wall shape pairs
    fn should_generate_contacts(&self, other: &ObjectType) -> bool {
        self != other || *self != ObjectType::Wall
    }
}

impl Component for ObjectType {
    type Storage = VecStorage<Self>;
}

/// Primitive emitter.
///
/// Will emit new primitives based on the internal config.
pub struct Emitter<P> {
    pub location: P,
    pub last_emit: Instant,
    pub emission_interval: Duration,
    pub material: Material,
    pub emitted: u64,
}

impl<P> Component for Emitter<P>
where
    P: Send + Sync + 'static,
{
    type Storage = DenseVecStorage<Self>;
}

/// Internal graphics used for the primitive emission.
pub struct Graphics {
    pub mesh: Handle<Mesh>,
}

pub struct KillRate(pub f32);

impl Default for KillRate {
    fn default() -> Self {
        KillRate(1.0)
    }
}

pub struct Collisions(pub u32);

impl Default for Collisions {
    fn default() -> Self {
        Collisions(0)
    }
}
