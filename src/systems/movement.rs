use std::marker;

use amethyst::core::LocalTransform;
use amethyst::core::cgmath::{Vector3, Quaternion, Matrix2, Matrix3, Basis2};
use amethyst::ecs::{Entities, Fetch, Join, ReadStorage, System, WriteStorage};
use rhusics::collide::prelude3d::BodyPose3;
use rhusics::collide::prelude2d::BodyPose2;
use amethyst::shrev::{EventChannel, ReaderId};

pub struct MovementSystem3;

impl MovementSystem3 {
    pub fn new() -> Self {
        Self { }
    }
}

impl<'a> System<'a> for MovementSystem3 {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BodyPose3>,
        WriteStorage<'a, LocalTransform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, poses, mut transforms) = data;

        for (pose, transform) in (&poses, &mut transforms).join() {
            *transform = LocalTransform {
                translation: Vector3::new(pose.position().x, pose.position().y, pose.position().z),
                rotation: pose.rotation().clone(),
                scale: transform.scale,
            }
        }
    }
}

pub struct MovementSystem2;

impl MovementSystem2 {
    pub fn new () -> Self {
        Self { }
    }
}

impl<'a> System<'a> for MovementSystem2 {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BodyPose2>,
        WriteStorage<'a, LocalTransform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, poses, mut transforms) = data;

        for (pose, transform) in (&poses, &mut transforms).join() {
            println!("{:?}", pose.position());
            let rot: Matrix3<f32> = Matrix2::<f32>::from(pose.rotation().clone() as Basis2<f32>)
                .into();

            *transform = LocalTransform {
                translation: Vector3::new(pose.position().x, pose.position().y, 0.),
                rotation: Quaternion::<f32>::from(rot),
                scale: transform.scale,
            }
        }
    }
}
