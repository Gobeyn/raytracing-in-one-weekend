use crate::vector::vector::Color;
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
/// See: https://netpbm.sourceforge.net/doc/ppm.html
pub fn add_ppm_header(file: &mut std::fs::File, img_width: i32, img_height: i32) {
    match file.write_all(format!("P3\n{} {}\n255\n", img_width, img_height).as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error writing to file: {err}");
            std::process::exit(1);
        }
    }
}
/// Write `Color` to image file as required by the plain PPM file format.
/// See: https://netpbm.sourceforge.net/doc/ppm.html
pub fn write_color(file: &mut std::fs::File, color: &Color) {
    // Transform [0,1] f64 values into [0,255] i32 values
    let ir: i32 = (255.999 * color.x) as i32;
    let ig: i32 = (255.999 * color.y) as i32;
    let ib: i32 = (255.999 * color.z) as i32;

    // Write to RGB color to image file.
    match file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error writing colors to image: {err}");
            std::process::exit(1);
        }
    }
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
}
