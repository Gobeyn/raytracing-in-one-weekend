use super::record::HitRecord;
use crate::raycaster::ray::Ray;
use crate::util::utils::Interval;

/// Hittable traits are able to implement the `ray_hit` method, meaning there is a way to determine
/// if a ray hit the object. The function should return a `HitRecord`.
pub trait Hittable {
    fn ray_hit(&self, ray: &Ray, ray_parameter_interval: Interval) -> HitRecord;
}

/// Create a struct that contains a vector of hittable objects. The hittable objects are those
/// structs that implement the `Hittable` trait.
/// Note: The elements of the vector must be contained in a `Box`, e.g. we need to surround each
/// entry of such a vector by `Box::new(...)`.
pub struct Hittables {
    hittable_list: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    /// Create new instance of `Hittables`
    pub fn new(hittable_list: Vec<Box<dyn Hittable>>) -> Self {
        Self { hittable_list }
    }
    /// Add element to the `Hittables.hittable_list`
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittable_list.push(hittable);
    }
}

impl Hittable for Hittables {
    /// Implement the `Hittable` trait for `Hittables`. We loop over all the elements and see if
    /// any of them hit. We can use the `Hittable` trait on all the elements as this is assumed to
    /// be the case. If there are multiple hits, the closest hit is returned.
    fn ray_hit(&self, ray: &Ray, ray_parameter_interval: Interval) -> HitRecord {
        // Get the default `HitRecord`
        let mut hit_record: HitRecord = HitRecord::default();
        // Initialise the current closest hit to the maximum allowed ray parameter.
        let mut closest_ray: f64 = ray_parameter_interval.max;

        // Loop over all the hittables
        for hittable in &self.hittable_list {
            // Get the hit record
            let current_hit_record = hittable.ray_hit(ray, ray_parameter_interval);
            // Check if it was a hit
            if current_hit_record.hit {
                // If so, check if the ray was closer than the current closest.
                if current_hit_record.ray_parameter <= closest_ray {
                    // If it was closer, update the closest ray and set the new hit record.
                    hit_record = current_hit_record;
                    closest_ray = current_hit_record.ray_parameter;
                }
            }
        }
        // Return the closest hit.
        return hit_record;
    }
}
