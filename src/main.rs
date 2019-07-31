mod vector;
mod ray;
mod sphere;
mod hit;
mod world;

use vector::Vector;
use ray::Ray;
use sphere::Sphere;
use world::World;
use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 800;
const HEIGHT: usize = 400;

fn pixel_to_coordinate(pixel: usize, width: usize, height: usize) -> (usize, usize) {
    (pixel % width, height - 1 - pixel / width)
}

fn color(ray: Ray, world: &World) -> Vector {
    if let Some(record) = world.hit(ray, 0.0, std::f64::MAX) {
        0.5 * (record.normal + Vector::ones())
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)*Vector::ones() + t*Vector::new(0.5, 0.7, 1.0)
    }
}

fn render(world: &World, width: usize, height: usize) -> Vec<u32> {
    let lower_left_corner = Vector::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::new(4.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, 2.0, 0.0);
    let origin = Vector::zeros();

    (0..width * height).map(|pixel| {
        let (x, y) = pixel_to_coordinate(pixel, width, height);
        let (u, v) = (x as f64 / width as f64, y as f64 / height as f64);
        let ray = Ray::new(
            origin,
            lower_left_corner + u*horizontal + v*vertical,
        );
        let col = color(ray, &world);
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
    
    let world = World::new(vec![
        Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0),
    ]);
    let buffer = render(&world, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap_or_else(|e| panic!("{}", e));
    }
}