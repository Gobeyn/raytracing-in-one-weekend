use crate::vector::vector::{Color, Vec3};
use rand::prelude::*;
use std::io::Write;

// Define useful constants.
pub const POSITIVE_INFINITY: f64 = std::f64::MAX;
pub const NEGATIVE_INFINITY: f64 = std::f64::MIN;
// If we could, we would set these as constants.
//pub const EMPTY: Interval = Interval::new(POSITIVE_INFINITY, NEGATIVE_INFINITY);
//pub const UNIVERSE: Interval = Interval::new(NEGATIVE_INFINITY, POSITIVE_INFINITY);

/// Create ./result/ directory if it does not exist.
pub fn create_result_dir() {
    match std::fs::create_dir("result") {
        Ok(_) => {}
        Err(err) => {
            log::info!("`result` directory already exists: {err}");
        }
    }
}
/// Add heading for identification of plain PPM files along with image dimensions.
/// See: <https://netpbm.sourceforge.net/doc/ppm.html>
pub fn add_ppm_header(file: &mut std::fs::File, img_width: i32, img_height: i32) {
    match file.write_all(format!("P3\n{} {}\n255\n", img_width, img_height).as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error writing to file: {err}");
            std::process::exit(1);
        }
    }
}
/// Conversion from linear to gamma, this is an implementation of the inverse `gamma 2` transform
pub fn linear_to_gamma(linear_value: f64) -> f64 {
    if linear_value > 0.0 {
        return linear_value.sqrt();
    } else {
        return 0.0;
    }
}
/// Write `Color` to image file as required by the plain PPM file format.
/// See: <https://netpbm.sourceforge.net/doc/ppm.html>
pub fn write_color(file: &mut std::fs::File, color: &Color) {
    // Define intensity interval.
    let intensity: Interval = Interval::new(0.0, 0.999);
    // Apply linear to gamma transform
    let r: f64 = linear_to_gamma(color.x);
    let g: f64 = linear_to_gamma(color.y);
    let b: f64 = linear_to_gamma(color.z);

    // Transform [0,1] f64 values into [0,255] i32 values
    let ir: i32 = (256.0 * intensity.clamp(r)) as i32;
    let ig: i32 = (256.0 * intensity.clamp(g)) as i32;
    let ib: i32 = (256.0 * intensity.clamp(b)) as i32;

    // Write to RGB color to image file.
    match file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error writing colors to image: {err}");
            std::process::exit(1);
        }
    }
}
/// Convert degrees into radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

/// Get a random `f64` between 0 and 1.
pub fn get_random() -> f64 {
    let mut rng = rand::thread_rng();
    let val: f64 = rng.gen();
    return val;
}
/// Get a random `f64` within the range [min, max].
pub fn get_random_in_range(min: f64, max: f64) -> f64 {
    return min + (max - min) * get_random();
}
/// Get random `Vec3` within the (-0.5, -0.5)-(0.5, 0.5) unit square.
pub fn sample_square() -> Vec3 {
    return Vec3::new(get_random() - 0.5, get_random() - 0.5, 0.0);
}

/// Struct that contains a minimum and maximum value
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    /// By default, `Interval` is set to $[- \infty, + \infty]$.
    fn default() -> Self {
        Self {
            min: NEGATIVE_INFINITY,
            max: POSITIVE_INFINITY,
        }
    }
}

impl Interval {
    /// Create new `Interval` instance.
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    /// Get the size of the interval, e.g. size([a, b]) = b - a.
    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }
    /// Check if a given value `x` lies within `Interval`, including the bounds.
    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }
    /// Check if a given value `x` lies within `Interval`, excluding the bounds.
    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }
    /// If the given value `x` falls outside the `Interval`, set it to the minimum
    /// or maximum value depending on where it fell outside the `Interval`.
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        } else {
            return x;
        }
    }
}
