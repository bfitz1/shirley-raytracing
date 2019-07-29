mod vector;
mod ray;
mod sphere;
mod hit;

use vector::Vector;
use ray::Ray;
use sphere::Sphere;
use hit::{HitRecord, HitableList, Hitable};
use minifb::{Key, Scale, WindowOptions, Window};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn color(ray: &Ray, world: &HitableList) -> Vector {
    if let Some(record) = world.hit(ray, 0.0, std::f64::MAX) {
        0.5 * Vector::new(
            record.normal.x + 1.0,
            record.normal.y + 1.0,
            record.normal.z + 1.0
        )
    } else {
        let direction = ray.direction.unit();
        let t = 0.5 * (direction.y + 1.0);
        (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0)
    }
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
    let world = HitableList::new(vec![
        Box::new(Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (p, i) in buffer.iter_mut().enumerate() {
            let u = (p % WIDTH) as f64 / WIDTH as f64;
            let v = (HEIGHT - (p / WIDTH) - 1) as f64 / HEIGHT as f64;
            let ray = Ray::new(
                Vector::zero(),
                corner + u * horizontal + v * vertical
            );

            let _p = ray.travel(2.0);
            let col = color(&ray, &world);

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
