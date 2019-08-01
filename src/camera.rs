use crate::vector::Vector;
use crate::ray::Ray;
use rand::{Rng, rngs::ThreadRng};

fn random_in_unit_disk(rng: &mut ThreadRng) -> Vector {
    loop {
        let p = 2.0 * Vector::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            0.0
        ) - Vector::new(1.0, 1.0, 0.0);
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

pub struct Camera {
    origin: Vector,
    corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
            lookfrom: Vector,
            lookat: Vector,
            vup: Vector,
            vfov: f64,
            aspect: f64,
            aperture: f64,
            focus_dist: f64
        ) -> Camera
    {
        let theta = vfov.to_radians();
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        
        Camera {
            origin: lookfrom,
            corner: lookfrom - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = Vector::new(u * rd.x, v * rd.y, 0.0);
        Ray::new(
            self.origin + offset,
            self.corner + u*self.horizontal + v*self.vertical - self.origin - offset
        )
    }
}