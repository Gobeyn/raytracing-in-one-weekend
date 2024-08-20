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
}
