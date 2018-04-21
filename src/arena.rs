use amethyst_core::cgmath::{Basis2, Deg, EuclideanSpace, One, Point2, Point3, Quaternion,
                            Rotation3};
use collision::{Aabb2, Aabb3, Line2};
use collision::primitive::{Primitive2, Primitive3, Quad};
use rhusics_core::{CollisionMode, CollisionShape, CollisionStrategy, RigidBody};
use rhusics_ecs::WithRigidBody;
use rhusics_ecs::physics2d::{BodyPose2, Mass2};
use rhusics_ecs::physics3d::{BodyPose3, Mass3};
use specs::World;

/// Setup 2D arena.
///
/// ### Parameters:
///
/// - `min`: Minimum corner of the arena
/// - `max`: Maximum corner of the arena
/// - `types`: Collider type of each arena barrier in order: Left, Right, Bottom, Top
/// - `world`: World
///
/// ### Type parameters:
///
/// - `Y`: Collider type
pub fn setup_2d_arena<Y>(min: Point2<f32>, max: Point2<f32>, types: (Y, Y, Y, Y), world: &mut World)
where
    Y: Default + Send + Sync + 'static,
{
    type Shape2<Y> = CollisionShape<Primitive2<f32>, BodyPose2<f32>, Aabb2<f32>, Y>;
    let center = (min + max.to_vec()) / 2.;
    world
        .create_entity()
        .with_static_rigid_body(
            Shape2::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Line2::new(Point2::new(center.x, min.y), Point2::new(center.x, max.y)).into(),
                types.0,
            ),
            Transform::new(Point2::new(min.x, center.y), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();

    world
        .create_entity()
        .with_static_rigid_body(
            Shape2::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Line2::new(Point2::new(center.x, min.y), Point2::new(center.x, max.y)).into(),
                types.1,
            ),
            Transform::new(Point2::new(max.x, center.y), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();

    world
        .create_entity()
        .with_static_rigid_body(
            Shape2::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Line2::new(Point2::new(min.x, center.y), Point2::new(max.x, center.y)).into(),
                types.2,
            ),
            Transform::new(Point2::new(center.x, min.y), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();

    world
        .create_entity()
        .with_static_rigid_body(
            Shape2::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Line2::new(Point2::new(min.x, center.y), Point2::new(max.x, center.y)).into(),
                types.3,
            ),
            Transform::new(Point2::new(center.x, max.y), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();
}

/// Setup 3D arena.
///
/// ### Parameters:
///
/// - `min`: Minimum corner of the arena
/// - `max`: Maximum corner of the arena
/// - `types`: Collider type of each arena barrier in order: Left, Right, Bottom, Top, Front, Back
/// - `world`: World
///
/// ### Type parameters:
///
/// - `Y`: Collider type
pub fn setup_3d_arena<Y>(
    min: Point3<f32>,
    max: Point3<f32>,
    types: (Y, Y, Y, Y, Y, Y),
    world: &mut World,
) where
    Y: Default + Send + Sync + 'static,
{
    type Shape3<Y> = CollisionShape<Primitive3<f32>, BodyPose3<f32>, Aabb3<f32>, Y>;
    let dimension = max - min;
    let center = (min + max.to_vec()) / 2.;
    println!(
        "Left: {}, {}, {}, {}, {}",
        dimension.z, dimension.y, min.x, center.y, center.z
    );
    world
        .create_entity()
        .with_static_rigid_body(
            Shape3::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Quad::new(dimension.z, dimension.y).into(),
                types.0,
            ),
            Transform::new(
                Point3::new(min.x, center.y, center.z),
                Quaternion::from_angle_y(Deg(90.)),
            ),
            RigidBody::default(),
            Mass3::infinite(),
        )
        .build();

    println!(
        "Right: {}, {}, {}, {}, {}",
        dimension.z, dimension.y, max.x, center.y, center.z
    );
    world
        .create_entity()
        .with_static_rigid_body(
            Shape3::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Quad::new(dimension.z, dimension.y).into(),
                types.1,
            ),
            Transform::new(
                Point3::new(max.x, center.y, center.z),
                Quaternion::from_angle_y(Deg(90.)),
            ),
            RigidBody::default(),
            Mass3::infinite(),
        )
        .build();

    println!(
        "Bottom: {}, {}, {}, {}, {}",
        dimension.x, dimension.z, center.x, min.y, center.z
    );
    world
        .create_entity()
        .with_static_rigid_body(
            Shape3::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Quad::new(dimension.x, dimension.z).into(),
                types.2,
            ),
            Transform::new(
                Point3::new(center.x, min.y, center.z),
                Quaternion::from_angle_x(Deg(90.)),
            ),
            RigidBody::default(),
            Mass3::infinite(),
        )
        .build();

    println!(
        "Top: {}, {}, {}, {}, {}",
        dimension.x, dimension.z, center.x, max.y, center.z
    );
    world
        .create_entity()
        .with_static_rigid_body(
            Shape3::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Quad::new(dimension.x, dimension.z).into(),
                types.3,
            ),
            Transform::new(
                Point3::new(center.x, max.y, center.z),
                Quaternion::from_angle_x(Deg(90.)),
            ),
            RigidBody::default(),
            Mass3::infinite(),
        )
        .build();

    println!(
        "Front: {}, {}, {}, {}, {}",
        dimension.x, dimension.y, center.x, center.y, max.z
    );
    world
        .create_entity()
        .with_static_rigid_body(
            Shape3::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Quad::new(dimension.x, dimension.y).into(),
                types.4,
            ),
            Transform::new(Point3::new(center.x, center.y, max.z), Quaternion::one()),
            RigidBody::default(),
            Mass3::infinite(),
        )
        .build();

    println!(
        "Back: {}, {}, {}, {}, {}",
        dimension.x, dimension.y, center.x, center.y, min.z
    );
    world
        .create_entity()
        .with_static_rigid_body(
            Shape3::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Quad::new(dimension.x, dimension.y).into(),
                types.5,
            ),
            Transform::new(Point3::new(center.x, center.y, min.z), Quaternion::one()),
            RigidBody::default(),
            Mass3::infinite(),
        )
        .build();
}
