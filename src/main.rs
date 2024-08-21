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
use materials::materials::{Dielectric, Lambertian, Metal};
use util::utils;
use vector::vector::{Color, Point, Vec3};
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
    // 1080p resolution (1080 x 1920) with 16:9 aspect ratio
    //let image_width = 1920;
    let image_width = 400;
    // Define center of camera
    let camera_center: Point = Point::new(13.0, 2.0, 3.0);
    // Define the amount of samples per pixel
    let samples_per_pixel: i32 = 100;
    // Define maximum amount of bounces the ray can do.
    let max_depth: i32 = 50;
    // Define vfov (vertical field of view)
    let vfov: f64 = 20.0;
    // Define the point we are looking at (center if the canvas)
    let look_at: Point = Point::new(0.0, 0.0, 0.0);
    // Define vup, the Camera relative up direction
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    // Define defocus parameters
    let defocus_angle: f64 = 0.6;
    let focus_dist: f64 = 10.0;

    // Define Camera instance
    let camera: Camera = Camera::initialize(
        aspect_ratio,
        image_width,
        camera_center,
        samples_per_pixel,
        max_depth,
        vfov,
        look_at,
        vup,
        defocus_angle,
        focus_dist,
    );

    // Define the world -- cover image
    let mut world: Hittables = Hittables::init();

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = utils::get_random();
            let sphere_center: Point = Point::new(
                a as f64 + 0.9 * utils::get_random(),
                0.2,
                b as f64 + 0.9 * utils::get_random(),
            );

            if (sphere_center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::get_random_vector() * Color::get_random_vector();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(sphere_center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::get_random_vector_in_range(0.5, 1.0);
                    let fuzz = utils::get_random_in_range(0.5, 1.0);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(sphere_center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5);
                    world.add(Box::new(Sphere::new(sphere_center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5);
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    // Define the world -- Ground ball, Glass ball, Matt ball and Metal ball.
    //let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    //let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    //let material_left = Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.50);
    //let material_bubble = Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.0 / 1.50);
    //let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    //
    //let mut world: Hittables = Hittables::init();
    //world.add(Box::new(Sphere::new(
    //    Point::new(0.0, -100.5, -1.0),
    //    100.0,
    //    material_ground,
    //)));
    //world.add(Box::new(Sphere::new(
    //    Point::new(0.0, 0.0, -1.2),
    //    0.5,
    //    material_center,
    //)));
    //world.add(Box::new(Sphere::new(
    //    Point::new(-1.0, 0.0, -1.0),
    //    0.5,
    //    material_left,
    //)));
    //world.add(Box::new(Sphere::new(
    //    Point::new(-1.0, 0.0, -1.0),
    //    0.4,
    //    material_bubble,
    //)));
    //world.add(Box::new(Sphere::new(
    //    Point::new(1.0, 0.0, -1.0),
    //    0.5,
    //    material_right,
    //)));
    //
    // Render image
    camera.render(&mut file, &world);
}
