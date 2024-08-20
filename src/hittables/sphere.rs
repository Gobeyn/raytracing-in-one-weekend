use super::hittables::Hittable;
use super::record::{set_face_normal, HitRecord};
use crate::raycaster::ray::Ray;
use crate::util::utils::Interval;
use crate::vector::vector::{Point, Vec3};

/// A `Sphere` is defined by the location of its center in 3D space, and the radius of it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    /// Create new `Sphere` instance.
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    /// Given a sphere and a line in 3D, one can perform some math to find the conditions for that
    /// line to intersect the sphere. This method simply implements that math and returns if the
    /// line intersects or not. By replacing `b = -2h` in the quadratic formula, the implementation
    /// becomes even simpler.
    fn ray_hit(&self, ray: &Ray, ray_parameter_interval: Interval) -> HitRecord {
        let oc: Vec3 = self.center - ray.origin;
        let a: f64 = ray.direction.length_squared();
        let h: f64 = ray.direction.dot(&oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = h * h - a * c;

        // No solution to quadratic, so ray missed.
        if discriminant < 0.0 {
            return HitRecord::default();
        }
        let sqrt_d: f64 = discriminant.sqrt();
        // Find nearest root in the acceptable range.
        let root: f64 = {
            let root_minus = (h - sqrt_d) / a;
            if !ray_parameter_interval.surrounds(root_minus) {
                // If we get here, the minus root did not lie in the acceptable range.
                let root_plus = (h + sqrt_d) / a;
                if !ray_parameter_interval.surrounds(root_plus) {
                    // If we get here, the plus root also did not lie in the acceptable range,
                    // so the ray did not hit.
                    return HitRecord::default();
                } else {
                    // If we get here, the plus root did lie in the acceptable range, and the
                    // minus root has already been ruled out, so root takes the value of root_plus.
                    root_plus
                }
            } else {
                // If we get here, the minus root did lie in the acceptable range, and it
                // is the closest, so root takes the value of root_minus.
                root_minus
            }
        };

        // Set the fields of the hit record.
        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let (front_face, normal) = set_face_normal(ray, outward_normal);
        return HitRecord::new(true, point, normal, front_face, root);
    }
}
