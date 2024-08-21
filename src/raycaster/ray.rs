use crate::camera::camera::Camera;
use crate::hittables::hittables::Hittable;
use crate::hittables::hittables::Hittables;
use crate::hittables::record::HitRecord;
use crate::util::utils::sample_square;
use crate::util::utils::Interval;
use crate::util::utils::POSITIVE_INFINITY;
use crate::vector::vector::{Color, Point, Vec3};

/// A `Ray` is defined is effectively a line in 3D. This line can be fully defined by a
/// point (the origin) and a vector from that point (the direction). Effectively it is a function
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    /// Create new `Ray` instance.
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    /// The `Ray` structure defines the constants needed to define a parametrization of a
    /// line in 3D. To actually obtain a point along this line the parameter value, here called
    /// `scalar`, must be provided. This method effectively captures the function definition of a
    /// ray:
    /// $$ \vec{P}(t) = \vec{A} + t \vec{b}, $$
    /// with $\vec{A}$ the origin, $\vec{b}$ the direction and $t$ the parametrization scalar.
    pub fn at(&self, scalar: f64) -> Vec3 {
        return self.origin + self.direction * scalar;
    }
    /// Send the given `Ray` out into the `world`, if it hits a `Hittable` object, do something
    /// with the colors. If it does not hit anything, do the default coloring.
    pub fn ray_color(&self, world: &Hittables) -> Color {
        let hit_record: HitRecord = world.ray_hit(self, Interval::new(0.0, POSITIVE_INFINITY));

        if hit_record.hit {
            return Color::new(
                hit_record.normal.x + 1.0,
                hit_record.normal.y + 1.0,
                hit_record.normal.z + 1.0,
            ) * 0.5;
        }

        let unit_direction = self.direction.unit_vector();
        let a: f64 = (unit_direction.y + 1.0) * 0.5;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }
    /// Given a pixel location (i,j), shoot a ray from the `Camera` to a random
    /// location within the pixel square.
    pub fn get_ray(i: i32, j: i32, camera: &Camera) -> Self {
        let offset: Vec3 = sample_square();
        let pixel_sample = camera.pixel_upper_left_center
            + (camera.pixel_delta_u * (i as f64 + offset.x))
            + (camera.pixel_delta_v * (j as f64 + offset.y));
        let ray_origin: Point = camera.center;
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        return Self::new(ray_origin, ray_direction);
    }
}
