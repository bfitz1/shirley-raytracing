mod vector;
mod ray;
mod sphere;
mod hit;
mod camera;

use vector::Vector;
use ray::Ray;
use sphere::Sphere;
use hit::{HitableList, Hitable};
use camera::Camera;

use minifb::{Key, Scale, WindowOptions, Window};
use rand::prelude::*;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const SAMPLES: usize = 100;

fn random_in_unit_sphere() -> Vector {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vector::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            rng.gen::<f64>()
        ) - Vector::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn color(ray: &Ray, world: &HitableList) -> Vector {
    if let Some(record) = world.hit(ray, 0.001, std::f64::MAX) {
        let target = record.p + record.normal + random_in_unit_sphere();
        0.5 * color(&Ray::new(record.p, target - record.p), world)
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

    let world = HitableList::new(vec![
        Box::new(Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0)),
    ]);
    let cam = Camera::default();
    let mut rng = rand::thread_rng();

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (p, i) in buffer.iter_mut().enumerate() {
            let mut col = Vector::zero();
            for _ in 0..SAMPLES {
                let i = (p % WIDTH) as f64;
                let j = (HEIGHT - (p / WIDTH) - 1) as f64;
                let u = (i + rng.gen::<f64>()) / WIDTH as f64;
                let v = (j + rng.gen::<f64>()) / HEIGHT as f64;
                let r = cam.get_ray(u, v);
                let _p = r.travel(2.0);
                col += color(&r, &world);
            }
            col /= SAMPLES as f64;
            col = Vector::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
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
