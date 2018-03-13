use amethyst_core::GlobalTransform;
use amethyst_core::cgmath::{EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Transform,
                            Vector3, Vector4};
use amethyst_renderer::Camera;
use collision::Ray3;

fn mouse_to_clip(x: f32, y: f32, width: f32, height: f32) -> Vector4<f32> {
    Vector4::new((2.0 * x) / width - 1.0, 1.0 - (2.0 * y) / height, -1.0, 1.0)
}

fn clip_to_eye(clip: Vector4<f32>, inverse_projection: Matrix4<f32>) -> Vector4<f32> {
    let eye = inverse_projection * clip;
    Vector4::new(eye.x, eye.y, -1.0, 0.0)
}

fn eye_to_world(eye: Vector4<f32>, inverse_view: Matrix4<f32>) -> Vector3<f32> {
    (inverse_view * eye).truncate().normalize()
}

/// Generate a ray for picking, based on the clicked position in pixel coordinates.
pub fn pick_ray(
    window_pos: (f32, f32), // pixel coordinates
    window_size: (f32, f32),
    camera: &Camera,
    view: &GlobalTransform,
) -> Ray3<f32> {
    let clip = mouse_to_clip(window_pos.0, window_pos.1, window_size.0, window_size.1);
    let eye = clip_to_eye(clip, camera.proj.invert().unwrap());
    let world_dir = eye_to_world(eye, view.0.invert().unwrap());
    Ray3::new(view.0.transform_point(Point3::origin()), world_dir)
}

/// Generate a ray for picking, based on the clicked position in normalized device coordinates.
pub fn pick_ray_ndc(
    window_pos: (f32, f32), // ndc coordinates (-1..1)
    camera: &Camera,
    view: &GlobalTransform,
) -> Ray3<f32> {
    let clip = Vector4::new(window_pos.0, window_pos.1, -1.0, 1.0);
    let eye = clip_to_eye(clip, camera.proj.invert().unwrap());
    let world_dir = eye_to_world(eye, view.0.invert().unwrap());
    Ray3::new(view.0.transform_point(Point3::origin()), world_dir)
}
