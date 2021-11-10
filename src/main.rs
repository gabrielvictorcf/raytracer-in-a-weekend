mod vec3;
mod color;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use camera::Camera;
use color::Color;
use hit::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::{Point3, Vec3};

use std::sync::Arc;

use image::RgbImage;
use rayon::iter::{ParallelBridge, ParallelIterator};

// Image constants
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: f64 = 1280.0;
const IMG_HEIGHT: f64 = IMG_WIDTH / ASPECT_RATIO;
const PIXEL_SAMPLES: f64 = 500.0;
const MAX_RAY_BOUNCES: u32 = 50;

fn main() {
    // World/Scene initialization
    let world = random_scene();

    // Instantiate Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);
    
    // Setup our PNG RgbImage and get an iterator to its rows
    let mut img = RgbImage::new(IMG_WIDTH as u32, IMG_HEIGHT as u32);
    let mut rows = img.rows_mut();

    // Rendering main loop - iterate over rows and pixels then shoot our rays
    for j in (0..IMG_HEIGHT as usize).rev() {
        eprintln!("On row {}", j);
        let row = rows.next().unwrap().enumerate();

        row.par_bridge().for_each(|(i, img_pixel)| {
            let mut pixel = color::BLACK;

            let rng = fastrand::Rng::new();
            for _ in 0..PIXEL_SAMPLES as usize {
                let u = (i as f64 + rng.f64()) / (IMG_WIDTH - 1.0);
                let v = (j as f64 + rng.f64()) / (IMG_HEIGHT - 1.0);
                
                let ray = cam.gen_ray(u, v);
                pixel += world.find_ray_color(ray, MAX_RAY_BOUNCES);
            }

            *img_pixel = pixel.to_rgb(PIXEL_SAMPLES as f64);
        })
    }

    let path = std::env::args().nth(1).unwrap_or("ray".to_string());
    let path = format!("/home/cypherlock/images/{}.png", path);
    eprintln!("Saving image to path {}", path);
    if let Err(e) = img.save(path) {
        eprintln!("Error on saving image - {}", e);
        eprintln!("Defaulting to 'ray.png'");

        let mut cwd = std::env::current_dir().expect("Unable to get cwd");
        cwd.push("ray.png");
        img.save(cwd).expect("Unable to save with default name");
    }
}

fn random_scene() -> HittableList {
    // Init empty world
    let mut world = HittableList::default();

    // Make our ground sphere
    let albedo = Color::new(0.5, 0.5, 0.5);
    let mat_ground = Arc::new(Lambertian::new(albedo));
    let sphere_ground = Sphere::new(0.0, -1000.0, 0.0, 1000.0, mat_ground);
    world.add(sphere_ground);

    let p = Point3::new(4.0, 0.2, 0.0);

    let rng = fastrand::Rng::new();
    for i in -11..11 {
        for j in -11..11 {
            let i = i as f64;
            let j = j as f64;

            let rand_mat = rng.f64();
            let center = Point3::new(i + 0.9 * rng.f64(), 0.2, j + 0.9 * rng.f64());

            if (center - p).len() < 0.9 {
                continue;
            }

            let material: Arc<dyn material::Material> = match rand_mat {
                x if x < 0.8 => {   // Diffuse
                    let albedo = Color::rand() * Color::rand();
                    Arc::new(Lambertian::new(albedo))
                },
                x if x < 0.95 => {  // Metal
                    let albedo = Color::rand_range(0.5..1.0);
                    let fuzz = rng.f64() * 0.5; // f64 in range 0.0..0.5
                    Arc::new(Metal::new(albedo, fuzz))
                },
                _ => Arc::new(Dielectric::new(1.5)) // Glass
            };

            let sphere = Sphere::new(center.x, center.y, center.z, 0.2, material);
            world.add(sphere);
        }
    }

    // Instantiate the big glass sphere.
    let material = Arc::new(Dielectric::new(1.5));
    let sphere = Sphere::new(0.0, 1.0, 0.0, 1.0, material);
    world.add(sphere);

    // Instantiate the big opaque sphere.
    let albedo = Color::new(0.4, 0.2, 0.1);
    let material = Arc::new(Lambertian::new(albedo));
    let sphere = Sphere::new(-4.0, 1.0, 0.0, 1.0, material);
    world.add(sphere);

    // Instantiate the big metallic sphere.
    let albedo = Color::new(0.7, 0.6, 0.5);
    let fuzz = 0.0;
    let material = Arc::new(Metal::new(albedo, fuzz));
    let sphere = Sphere::new(4.0, 1.0, 0.0, 1.0, material);
    world.add(sphere);

    world
}