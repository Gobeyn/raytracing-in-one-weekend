use crate::vector::vector::Color;
use std::io::Write;

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
