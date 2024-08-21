use crate::hittables::record::HitRecord;
use crate::raycaster::ray::Ray;
use crate::vector::vector::{Color, Vec3};

/// Information structure about scattered ray, namely, if the ray scattered,
/// if so, what the new ray is and the color attenuation factor the scattering.
#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub did_scatter: bool,
    pub ray: Ray,
    pub attenuation: Color,
}

impl Scatter {
    /// Create new instance of `Scatter`.
    pub fn new(did_scatter: bool, ray: Ray, attenuation: Color) -> Self {
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
/// color. Albedo is Latin for whiteness and in this context defines the fractional
/// reflectance.
#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    /// Create new instance of `Lambertian`
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl Material for Lambertian {
    /// A `Lambertian` material scatters light back in a random direction following
    /// a Lambertian distribution. We assume constant attenuation.
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

/// A `Metal` material is defined by the fact that it reflects light. The color of the
/// light is attenuated by the `albedo`. Albedo is Latin for whiteness and in this context defines the fractional
/// reflectance. The `fuzz` field is assumed to be a value in [0, 1].
#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    /// Create new instance of `Metal`.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    /// A `Metal` material scatters light by reflection with respect to the
    /// normal. We assume constant attenuation.
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Scatter {
        let scattered_direction: Vec3 = ray_in.direction.reflect(hit_record.normal)
            + Vec3::get_random_unit_vector() * self.fuzz;
        let scattered_ray: Ray = Ray::new(hit_record.point, scattered_direction);
        // Check if the scattered ray is going into the material, e.g. the
        // dot product with the normal is negative. If so, the ray is absorbed and
        // hence not scattered.
        let did_scatter: bool = scattered_direction.dot(&hit_record.normal) > 0.0;
        return Scatter {
            did_scatter,
            ray: scattered_ray,
            attenuation: self.albedo,
        };
    }
}
