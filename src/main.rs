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
            let ray = camera.get_ray(u, v, &mut rng);
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

fn random_scene(rng: &mut ThreadRng) -> World {
    let mut spheres = Vec::with_capacity(500);
    spheres.push(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian(Vector::new(0.5, 0.5, 0.5))
    ));

    let mut random = || rng.gen::<f64>();

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = random();
            let center = Vector::new(a + 0.9 * random(), 0.2, b + 0.9 * random());
            if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        Material::lambertian(Vector::new(random() * random(), random() * random(), random() * random()))
                    ));
                } else if choose_mat < 0.95 {
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        Material::metal(Vector::new(0.5 * (1.0 + random()), 0.5 * (1.0 + random()), 0.5 * (1.0 + random())), 0.5 * random())
                    ));
                } else {
                    spheres.push(Sphere::new(center, 0.2, Material::dielectric(1.5)));
                }
            }
        }
    }

    spheres.push(Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0, Material::dielectric(1.5)));
    spheres.push(Sphere::new(Vector::new(-4.0, 1.0, 0.0), 1.0, Material::lambertian(Vector::new(0.4, 0.2, 0.1))));
    spheres.push(Sphere::new(Vector::new(4.0, 1.0, 0.0), 1.0, Material::metal(Vector::new(0.7, 0.6, 0.5), 0.0)));

    World::new(spheres)
}

fn main() {
    let mut window = Window::new("Raytracer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    
    let lookfrom = Vector::new(3.0, 3.0, 2.0);
    let lookat = Vector::new(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        WIDTH as f64 / HEIGHT as f64,
        aperture,
        dist_to_focus
    );
    let world = World::new(vec![
        Sphere::new(
            Vector::new(0.0, 0.0, -1.0),
            0.5,
            Material::lambertian(Vector::new(0.1, 0.2, 0.5))),
        Sphere::new(
            Vector::new(0.0, -100.5, -1.0),
            100.0,
            Material::lambertian(Vector::new(0.8, 0.8, 0.0))),
        Sphere::new(
            Vector::new(1.0, 0.0, -1.0),
            0.5,
            Material::metal(Vector::new(0.8, 0.6, 0.2), 0.0)),
        Sphere::new(
            Vector::new(-1.0, 0.0, -1.0),
            0.5,
            Material::dielectric(1.5)),
        Sphere::new(
            Vector::new(-1.0, 0.0, -1.0),
            -0.45,
            Material::dielectric(1.5)),
    ]);
    let buffer = render(&camera, &world, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap_or_else(|e| panic!("{}", e));
    }
}