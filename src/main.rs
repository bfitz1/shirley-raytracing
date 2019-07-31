mod vector;
mod ray;
mod sphere;
mod hit;
mod world;
mod camera;
mod material;

use vector::Vector;
use ray::Ray;
use sphere::Sphere;
use world::World;
use camera::Camera;
use material::*;

use minifb::{Key, WindowOptions, Window};
use rand::prelude::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 400;
const SAMPLES: usize = 100;

fn pixel_to_coordinate(pixel: usize, width: usize, height: usize) -> (usize, usize) {
    (pixel % width, height - 1 - pixel / width)
}

fn color(ray: Ray, world: &World, depth: usize, rng: &mut ThreadRng) -> Vector {
    if let Some(record) = world.hit(ray, 0.001, std::f64::MAX) {
        match record.material.scatter(ray, record, rng) {
            Some(s) if depth < 50 => s.attenuation * color(s.scattered, world, depth + 1, rng),
            _ => Vector::zeros(),
        }
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)*Vector::ones() + t*Vector::new(0.5, 0.7, 1.0)
    }
}

fn render(camera: &Camera, world: &World, width: usize, height: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    (0..width * height).map(|pixel| {
        let (x, y) = pixel_to_coordinate(pixel, width, height);
        let mut col = Vector::zeros();
        for _ in 0..SAMPLES {
            let (u, v) = (
                (x as f64 + rng.gen::<f64>()) / width as f64,
                (y as f64 + rng.gen::<f64>()) / height as f64
            );
            let ray = camera.get_ray(u, v);
            col += color(ray, world, 0, &mut rng);
        }
        let col = col / SAMPLES as f64;
        let col = Vector::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
        let (ir, ig, ib) = (
            (255.99 * col.x) as u8,
            (255.99 * col.y) as u8,
            (255.99 * col.z) as u8,
        );
        u32::from_be_bytes([0, ir, ig, ib])
    }).collect()
}

fn main() {
    let mut window = Window::new("Raytracer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    
    let camera = Camera::default();
    let world = World::new(vec![
        Sphere::new(
            Vector::new(0.0, 0.0, -1.0),
            0.5,
            Material::lambertian(Vector::new(0.8, 0.3, 0.3))),
        Sphere::new(
            Vector::new(0.0, -100.5, -1.0),
            100.0,
            Material::lambertian(Vector::new(0.8, 0.8, 0.0))),
        Sphere::new(
            Vector::new(1.0, 0.0, -1.0),
            0.5,
            Material::metal(Vector::new(0.8, 0.6, 0.2), 1.0)),
        Sphere::new(
            Vector::new(-1.0, 0.0, -1.0),
            0.5,
            Material::metal(Vector::new(0.8, 0.8, 0.8), 0.3)),
    ]);
    let buffer = render(&camera, &world, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap_or_else(|e| panic!("{}", e));
    }
}