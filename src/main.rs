pub mod camera;
pub mod hittables;
pub mod logger;
pub mod materials;
pub mod raycaster;
pub mod util;
pub mod vector;

// Internal files
use camera::camera::Camera;
use hittables::hittables::Hittables;
use hittables::sphere::Sphere;
use logger::logger::init_logging;
use util::utils;
use vector::vector::Point;
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

    // Define aspect ratio, which is defined as the width/height.
    let aspect_ratio: f64 = 16.0 / 9.0;
    // Define image width
    // 480p resolution (854 x 480) with 16:9 aspect ratio
    //let image_width = 854;
    let image_width = 400;
    // Define center of camera
    let camera_center: Point = Point::new(0.0, 0.0, 0.0);
    // Define the amount of samples per pixel
    let samples_per_pixel: i32 = 100;
    // Define maximum amount of bounces the ray can do.
    let max_depth: i32 = 50;

    // Define Camera instance
    let camera: Camera = Camera::initialize(
        aspect_ratio,
        image_width,
        camera_center,
        samples_per_pixel,
        max_depth,
    );

    // Define the world
    let world: Hittables = Hittables::new(vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    // Render image
    camera.render(&mut file, &world);
}
