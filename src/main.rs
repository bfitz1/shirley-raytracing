mod vector;
mod ray;

use vector::Vector;
use ray::Ray;
use minifb::{Key, Scale, WindowOptions, Window};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn hit_sphere(center: Vector, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(ray: &Ray) -> Vector {
    if hit_sphere(Vector::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vector::new(1.0, 0.0, 0.0);
    }
    let direction = ray.direction.unit();
    let t = 0.5 * (direction.y + 1.0);
    (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0)
}

fn main() {
    let opts = WindowOptions { scale: Scale::X4, ..WindowOptions::default() };
    let mut window = match Window::new("Raytracer", WIDTH, HEIGHT, opts) {
        Ok(win) => win,
        Err(err) => panic!("Unable to create window: {}", err),
    };

    let corner = Vector::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::new(4.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, 2.0, 0.0);

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (p, i) in buffer.iter_mut().enumerate() {
            let u = (p % WIDTH) as f64 / WIDTH as f64;
            let v = (HEIGHT - (p / WIDTH) - 1) as f64 / HEIGHT as f64;
            let ray = Ray::new(
                Vector::zero(),
                corner + u * horizontal + v * vertical
            );
            let col = color(&ray);

            *i = u32::from_be_bytes([
                0x00,
                (col.x * 255.99) as u8,
                (col.y * 255.99) as u8,
                (col.z * 255.99) as u8
            ]);
        }
        window.update_with_buffer(&buffer).unwrap();
    }
}
