use amethyst_core::cgmath::{Basis2, Deg, EuclideanSpace, One, Point2, Point3, Quaternion,
                            Rotation3, Vector2};
use amethyst_core::specs::prelude::World;
use collision::primitive::{Primitive2, Primitive3, Quad};
use collision::{Aabb2, Aabb3, Line2};
use rhusics_core::{CollisionMode, CollisionShape, CollisionStrategy, Pose, RigidBody};
use rhusics_ecs::WithRigidBody;
use rhusics_ecs::physics2d::{BodyPose2, Mass2};
use rhusics_ecs::physics3d::{BodyPose3, Mass3};

/// Setup 3D arena.
///
/// An arena is a space with invisible walls around it, which have collision shapes defined.
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
    let dimension = max - min;
    let center = (min + max.to_vec()) / 2.;
    create_3d_wall(
        world,
        Vector2::new(dimension.z, dimension.y),
        types.0,
        Point3::new(min.x, center.y, center.z),
        Quaternion::from_angle_y(Deg(90.)),
    );
    create_3d_wall(
        world,
        Vector2::new(dimension.z, dimension.y),
        types.1,
        Point3::new(max.x, center.y, center.z),
        Quaternion::from_angle_y(Deg(90.)),
    );
    create_3d_wall(
        world,
        Vector2::new(dimension.x, dimension.z),
        types.2,
        Point3::new(center.x, min.y, center.z),
        Quaternion::from_angle_x(Deg(90.)),
    );
    create_3d_wall(
        world,
        Vector2::new(dimension.x, dimension.z),
        types.3,
        Point3::new(center.x, max.y, center.z),
        Quaternion::from_angle_x(Deg(90.)),
    );
    create_3d_wall(
        world,
        Vector2::new(dimension.x, dimension.y),
        types.4,
        Point3::new(center.x, center.y, max.z),
        Quaternion::one(),
    );
    create_3d_wall(
        world,
        Vector2::new(dimension.x, dimension.y),
        types.5,
        Point3::new(center.x, center.y, min.z),
        Quaternion::one(),
    );
}

/// Setup 2D arena.
///
/// An arena is a space with invisible walls around the space, that defines a collision room.
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
            BodyPose2::new(Point2::new(min.x, center.y), Basis2::one()),
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
            BodyPose2::new(Point2::new(max.x, center.y), Basis2::one()),
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
            BodyPose2::new(Point2::new(center.x, min.y), Basis2::one()),
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
            BodyPose2::new(Point2::new(center.x, max.y), Basis2::one()),
            RigidBody::default(),
            Mass2::infinite(),
        )
        .build();
}

fn create_3d_wall<Y>(
    world: &mut World,
    dimension: Vector2<f32>,
    t: Y,
    position: Point3<f32>,
    rot: Quaternion<f32>,
) where
    Y: Default + Send + Sync + 'static,
{
    println!(
        "Wall: {}, {}, {}, {}, {}",
        dimension.x, dimension.y, position.x, position.y, position.z
    );
    world
        .create_entity()
        .with_static_rigid_body(
            Shape3::new_simple_with_type(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Quad::new_impl(dimension).into(),
                t,
            ),
            BodyPose3::new(position, rot),
            RigidBody::default(),
            Mass3::infinite(),
        )
        .build();
}

type Shape3<Y> = CollisionShape<Primitive3<f32>, BodyPose3<f32>, Aabb3<f32>, Y>;
