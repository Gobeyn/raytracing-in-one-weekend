use crate::raycaster::ray::Ray;
use crate::vector::vector::{Point, Vec3};

/// Structure that stores the information when a hit occurs, such as the
/// point that was registered as a hit, the normal vector of that point and
/// the parameter for that point along the ray.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord {
    pub hit: bool,
    pub point: Point,
    pub normal: Vec3,
    pub front_face: bool,
    pub ray_parameter: f64,
}

impl Default for HitRecord {
    /// By default, everything that can be zero is set to zero and hit is set to `false`.
    fn default() -> Self {
        Self {
            hit: false,
            point: Point::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            ray_parameter: 0.0,
        }
    }
}

impl HitRecord {
    /// Create a new instance of `HitRecord`
    pub fn new(
        hit: bool,
        point: Point,
        normal: Vec3,
        front_face: bool,
        ray_parameter: f64,
    ) -> Self {
        Self {
            hit,
            point,
            normal,
            front_face,
            ray_parameter,
        }
    }
}

/// Given a ray and a normal pointing outward from the hittable object. Check the
/// direction of the normal relative to the ray. If they are pointing opposite to each
/// other (e.g. the dot product is negative), then the location that normal came from is
/// front facing. If the directions align, then the location that normal came from must
/// be at the back of the hittable. We then return whether it was front facing or not and
/// update the given normal accordingly.
/// Note: The `outward_normal` parameter is assumed to be of unit length.
pub fn set_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    // Front facing if the dot product is negative.
    let front_face: bool = ray.direction.dot(&outward_normal) < 0.0;
    let normal = {
        // If front facing, ray is already going against the ray.
        if front_face {
            outward_normal
        } else {
            // Otherwise, it was going with the ray, so we update.
            -outward_normal
        }
    };
    return (front_face, normal);
}
