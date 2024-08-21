use crate::hittables::hittables::Hittables;
use crate::raycaster::ray::Ray;
use crate::util::utils;
use crate::vector::vector::{Color, Point, Vec3};

use indicatif::ProgressBar;

/// Camera structure that stores the essential information about the camera and contains methods
/// for rendering the world through ray casting.
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub center: Point,
    pub samples_per_pixel: i32,
    pub image_height: i32,
    pub pixel_upper_left_center: Point,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel_sample_scale: f64,
}

impl Camera {
    /// Using only base information that cannot be inferred using other values,
    /// create a new instance of `Camera`.
    pub fn initialize(
        aspect_ratio: f64,
        image_width: i32,
        center: Point,
        samples_per_pixel: i32,
    ) -> Self {
        // Compute rendered image height from given width and aspect ratio
        let image_height = (image_width as f64) / aspect_ratio;
        let image_height = {
            if image_height < 1.0 {
                1
            } else {
                image_height as i32
            }
        };

        // Define viewport dimensions
        // Distance between viewport and camera center (eye point) is the focal length. Vector from
        // camera center to viewport center is assumed to be orthogonal.
        let focal_length: f64 = 1.0;
        // Define virtual viewport. This is a virtual rectangle in 3D space with the same aspect ratio
        // as the image. It is through the pixels of this screen the rays will be sent.
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * ((image_width as f64) / (image_height as f64));
        // Define viewport coordinate system (u,v). The x-axis points from left to right, but the
        // y-axis points from up to down.
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        // Compute horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: Vec3 = viewport_u / (image_width as f64);
        let pixel_delta_v: Vec3 = viewport_v / (image_height as f64);
        // Compute location of upper left pixel
        let viewport_upper_left: Point =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_upper_left_center: Point =
            viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        // Compute pixel sample scale from samples per pixel
        let pixel_sample_scale: f64 = 1.0 / (samples_per_pixel as f64);

        return Self {
            aspect_ratio,
            image_width,
            center,
            samples_per_pixel,
            image_height,
            pixel_upper_left_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel_sample_scale,
        };
    }

    /// Given a `world` of `Hittable` objects, render the scene using ray casting and
    /// save the resulting render in the provided `file`.
    pub fn render(&self, file: &mut std::fs::File, world: &Hittables) {
        // Write PPM identifier line
        utils::add_ppm_header(file, self.image_width, self.image_height);
        // Initialise progress bar
        println!("Scanlines remaining");
        let prog_bar = ProgressBar::new(self.image_height as u64);
        // Render each pixel
        for j in 0..self.image_height {
            // Increment progress bar
            prog_bar.inc(1);
            for i in 0..self.image_width {
                // Initialise color to black
                let mut color: Color = Color::new(0.0, 0.0, 0.0);
                // Loop through samples per pixel
                for _ in 0..self.samples_per_pixel {
                    // Get a ray
                    let ray = Ray::get_ray(i, j, self);
                    color += ray.ray_color(world);
                }
                // Write color to file
                color *= self.pixel_sample_scale;
                utils::write_color(file, &color);
            }
        }
        // Finish progress bar
        prog_bar.finish();
    }
}
