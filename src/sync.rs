use amethyst_core::LocalTransform;
use amethyst_core::cgmath::{Array, BaseFloat, Basis2, EuclideanSpace, Matrix3, Point2, Point3,
                            Quaternion, Rotation, Vector3};
use amethyst_core::timing::Time;
use rhusics_core::BodyPose;
use rhusics_ecs::DeltaTime;
use specs::World;

pub trait AsLocalTransform {
    fn as_local_transform(&self) -> LocalTransform;
}

pub trait Convert {
    type Output;
    fn convert(&self) -> Self::Output;
}

impl<S> Convert for Point2<S>
where
    S: BaseFloat,
{
    type Output = Vector3<f32>;

    fn convert(&self) -> Self::Output {
        Vector3::new(self.x.to_f32().unwrap(), self.y.to_f32().unwrap(), 0.)
    }
}

impl<S> Convert for Point3<S>
where
    S: BaseFloat,
{
    type Output = Vector3<f32>;

    fn convert(&self) -> Self::Output {
        Vector3::new(
            self.x.to_f32().unwrap(),
            self.y.to_f32().unwrap(),
            self.z.to_f32().unwrap(),
        )
    }
}

impl<S> Convert for Basis2<S>
where
    S: BaseFloat,
{
    type Output = Quaternion<f32>;

    fn convert(&self) -> Self::Output {
        Matrix3::new(
            self.as_ref()[0][0].to_f32().unwrap(),
            self.as_ref()[0][1].to_f32().unwrap(),
            0.,
            self.as_ref()[1][0].to_f32().unwrap(),
            self.as_ref()[1][1].to_f32().unwrap(),
            0.,
            0.,
            0.,
            1.,
        ).into()
    }
}

impl<S> Convert for Quaternion<S>
where
    S: BaseFloat,
{
    type Output = Quaternion<f32>;

    fn convert(&self) -> Self::Output {
        Quaternion::new(
            self.s.to_f32().unwrap(),
            self.v.x.to_f32().unwrap(),
            self.v.y.to_f32().unwrap(),
            self.v.z.to_f32().unwrap(),
        )
    }
}

impl<P, R> AsLocalTransform for BodyPose<P, R>
where
    P: EuclideanSpace + Convert<Output = Vector3<f32>>,
    P::Scalar: BaseFloat,
    R: Rotation<P> + Convert<Output = Quaternion<f32>>,
{
    fn as_local_transform(&self) -> LocalTransform {
        LocalTransform {
            translation: self.position().convert(),
            rotation: self.rotation().convert(),
            scale: Vector3::from_value(1.),
        }
    }
}

pub fn time_sync(world: &World) {
    let mut delta = world.write_resource::<DeltaTime<f32>>();
    let time = world.read_resource::<Time>();
    delta.delta_seconds = time.delta_seconds();
}
