use amethyst_core::Transform;
use amethyst_core::cgmath::{Array, BaseFloat, Basis2, EuclideanSpace, Matrix3, Point2, Point3,
                            Quaternion, Rotation, Vector3};
use amethyst_core::timing::Time;
use rhusics_core::BodyPose;
use rhusics_ecs::DeltaTime;
use specs::World;

/// Utility trait for converting a transform type into an amethyst `Transform`.
pub trait AsTransform {
    /// Convert to `Transform`
    fn as_transform(&self) -> Transform;
}

/// Utility trait for converting data between types.
///
/// Primarily used for mapping data into amethysts internal data formation in `Transform`,
/// for example converting between `Point2<S>` to `Vector3<f32>` (which is used inside `Transform`).
pub trait Convert {
    /// Output type of conversion
    type Output;
    /// Convert
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

impl<P, R> AsTransform for BodyPose<P, R>
where
    P: EuclideanSpace + Convert<Output = Vector3<f32>>,
    P::Scalar: BaseFloat,
    R: Rotation<P> + Convert<Output = Quaternion<f32>>,
{
    fn as_transform(&self) -> Transform {
        Transform {
            translation: self.position().convert(),
            rotation: self.rotation().convert(),
            scale: Vector3::from_value(1.),
        }
    }
}

/// Utility function to sync time management from amethysts view of time, to rhusics view of time.
pub fn time_sync(world: &World) {
    let mut delta = world.write_resource::<DeltaTime<f32>>();
    let time = world.read_resource::<Time>();
    delta.delta_seconds = time.delta_seconds();
}
