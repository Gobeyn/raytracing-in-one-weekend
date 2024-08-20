pub mod hittables;
pub mod logger;
pub mod raycaster;
pub mod util;
pub mod vector;

// External crates
use indicatif::ProgressBar;

// Internal files
use logger::logger::init_logging;
use raycaster::ray::Ray;
use util::utils;
use vector::vector::{Point, Vec3};

// Standard library

fn main() {
    // Initialise logger
    init_logging();

    // Create result directory if it doesn't exist.
    utils::create_result_dir();

    // Create and open file
    let mut file = match std::fs::File::create("result/image.ppm") {
        Ok(f) => f,
        Err(err) => {
            log::error!("Error creating or opening `result/image.ppm` file: {err}");
            std::process::exit(1);
        }
    };

    // Define aspect ratio, which is defined as the widht/height.
    let aspect_ratio: f64 = 16.0 / 9.0;
    // Define image width
    let img_width = 1080;
    // Determine height using aspect ratio and width. We also make sure the height is at least one.
    let img_height = (img_width as f64) / aspect_ratio;
    let img_height = {
        if img_height < 1.0 {
            1
        } else {
            img_height as i32
        }
    };

    // Distance between viewport and camera center (eye point) is the focal length. Vector from
    // camera center to viewport center is assumed to be orthogonal.
    let focal_length: f64 = 1.0;
    let camera_center: Point = Point::new(0.0, 0.0, 0.0);
    // Define virtual viewport. This is a virtual rectangle in 3D space with the same aspect ratio
    // as the image. It is through the pixels of this screen the rays will be sent.
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * ((img_width as f64) / (img_height as f64));

    // Define viewport coordinate system (u,v). The x-axis points from left to right, but the
    // y-axis points from up to down.
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // Compute horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = viewport_u / (img_width as f64);
    let pixel_delta_v: Vec3 = viewport_v / (img_height as f64);

    // Compute location of upper left pixel
    let viewport_upper_left: Point =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_upper_left_center: Point =
        viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Write PPM file identifier line
    utils::add_ppm_header(&mut file, img_width, img_height);

    // Initialise progress bar
    println!("Scan lines remaining");
    let prog_bar = ProgressBar::new(img_height as u64);

    // Create image
    for j in 0..img_height {
        // Increment progress bar
        prog_bar.inc(1);
        for i in 0..img_width {
            // Get pixel center in (u,v) coordinates
            let pixel_center: Point = pixel_upper_left_center
                + (pixel_delta_u * (i as f64))
                + (pixel_delta_v * (j as f64));
            // Get ray direction by pointing from camera center through the pixel center
            let ray_direction: Vec3 = pixel_center - camera_center;
            // Define a `Ray` with the camera center as origin and the computed direction as
            // direction.
            let ray: Ray = Ray::new(camera_center, ray_direction);
            // Compute the ray color
            let color = ray.ray_color();
            // Write color to file
            utils::write_color(&mut file, &color);
        }
    }

    // Finish progress bar
    prog_bar.finish();
}
