use crate::util::utils::{get_random, get_random_in_range};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Custom three dimensional vector structure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Type alias for `Vec3` structure. Used for representing points in 3D.
pub type Point = Vec3;
/// Type alias for `Vec3` structure. Used for representing RGB colors.
pub type Color = Vec3;

/// Implement addition of `Vec3` structures. This allows us to use the `+` symbol.
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
/// Implement subtraction of two `Vec3` structures. This allows us to use the `-` symbol.
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
/// Implement scalar multiplication of `Vec3` structure with `f64` value. This allows us to use the `*` symbol.
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, multiplier: f64) -> Self {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
        }
    }
}
/// Implement multiplication of two `Vec3` structures as point-wise multiplication.
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
/// Implement scalar division of `Vec3` structure with `f64` value. This allows us to use the `/`
/// symbol.
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, divisor: f64) -> Self {
        self * (1.0 / divisor)
    }
}
/// Implement `+=` symbol for two `Vec3` structures
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
/// Implement `-=` symbol for two `Vec3` structures
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
/// Implement `*=` symbol for multiplying `Vec3` structure with `f64` value
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, multiplier: f64) {
        *self = *self * multiplier;
    }
}
/// Implement `*=` symbol for multiplying `Vec3` structures with each other.
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
/// Implement `/=` symbol for dividing `Vec3` structure with `f64` value
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, divisor: f64) {
        *self = *self * (1.0 / divisor)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        self * (-1.0)
    }
}

impl Vec3 {
    /// Create new `Vec3` instance.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    /// Get Euclidean norm of `Vec3` squared.
    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    /// Get Euclidean norm of `Vec3`.
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }
    /// Get dot product between two `Vec3` structures.
    pub fn dot(&self, other: &Self) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
    /// Get cross product between two `Vec3` structures. Not that this operation is not symmetric!
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    /// Get normalised version of the `Vec3`.
    pub fn unit_vector(&self) -> Self {
        return *self / self.length();
    }
    /// Get random vector in unit square, e.g. all directions are random values in (0,1).
    pub fn get_random_vector() -> Self {
        Self {
            x: get_random(),
            y: get_random(),
            z: get_random(),
        }
    }
    /// Get random vector where the values in each direction are bounded by [min, max].
    pub fn get_random_vector_in_range(min: f64, max: f64) -> Self {
        Self {
            x: get_random_in_range(min, max),
            y: get_random_in_range(min, max),
            z: get_random_in_range(min, max),
        }
    }
    /// Get random vector in unit sphere by randomly sampling within the bounding square and
    /// returning only when the sample lies within the sphere.
    pub fn get_random_in_unit_sphere() -> Self {
        loop {
            let p: Self = Self::get_random_vector_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    /// Get random unit vector by rescaling the random vector inside the unit sphere so that it is
    /// normalised.
    pub fn get_random_unit_vector() -> Self {
        return Self::get_random_in_unit_sphere().unit_vector();
    }
    /// Get random vector on the same hemisphere as the provided `normal` vector. We sample a
    /// random vector, check if it aligns with the `normal` or not, and turn the vector around
    /// of it does not align.
    pub fn get_random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere: Self = Self::get_random_unit_vector();
        let dot_prod: f64 = on_unit_sphere.dot(&normal);
        if dot_prod > 0.0 {
            // Normal and random vector lie in the same hemisphere, we can just return the random
            // vector.
            return on_unit_sphere;
        } else {
            // Random vector lies in the opposite hemisphere. We can flip it into the right
            // hemisphere by flipping the vector around.
            return -on_unit_sphere;
        }
    }
    /// Check if the vector is effectively the zero vector, e.g. all of its components lie
    /// below a certain threshold value.
    pub fn near_zero(&self) -> bool {
        let epsilon: f64 = 1e-8;
        self.x.abs() < epsilon && self.y.abs() < epsilon && self.z.abs() < epsilon
    }
    /// Reflect vector with respect to `normal`
    pub fn reflect(&self, normal: Self) -> Self {
        return *self - normal * 2.0 * self.dot(&normal);
    }
    /// Refract vector with respect to `normal` and ratio of refractive indices. In this
    /// function, it is the refractive index of the incoming ray divided by the refractive
    /// index of the outgoing ray.
    pub fn refract(&self, normal: Self, refractive_index_fraction: f64) -> Self {
        let cos_theta: f64 = normal.dot(&self.neg()).min(1.0);
        let out_perp: Self = (*self + normal * cos_theta) * refractive_index_fraction;
        let out_parallel = -normal * (1.0 - out_perp.length_squared()).abs().sqrt();
        return out_perp + out_parallel;
    }
}
