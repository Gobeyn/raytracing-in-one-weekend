use crate::hittables::record::HitRecord;
use crate::raycaster::ray::Ray;
use crate::vector::vector::Vec3;

/// Information structure about scattered ray, namely, if the ray scattered,
/// if so, what the new ray is and the color attenuation factor the scattering.
#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub did_scatter: bool,
    pub ray: Ray,
    pub attenuation: f64,
}

impl Scatter {
    /// Create new instance of `Scatter`.
    pub fn new(did_scatter: bool, ray: Ray, attenuation: f64) -> Self {
        Self {
            did_scatter,
            ray,
            attenuation,
        }
    }
}

/// Any `Material` should implement what it means for a `Ray` to scatter on
/// that material.
pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Scatter;
}

/// A Lambertian material is essentially a diffuse material. The material scatters light
/// randomly according to a Lambertian distribution and attenuates according to the `albedo`
/// parameter. Albedo is Latin for whiteness and in this context defines the fractional
/// reflectance.
#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: f64,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Scatter {
        let scattering_direction = {
            let tmp = hit_record.normal + Vec3::get_random_unit_vector();

            // Catch the case where the normal vector and random vector happen to cancel
            //each other out resulting a zero vector. In that case, the scattering
            // direction should be the normal
            if tmp.near_zero() {
                hit_record.normal
            } else {
                tmp
            }
        };

        let scattered_ray = Ray::new(hit_record.point, scattering_direction);
        return Scatter {
            did_scatter: true,
            ray: scattered_ray,
            attenuation: self.albedo,
        };
    }
}
