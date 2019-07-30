use crate::vector::Vector;
use crate::ray::Ray;

pub struct Camera {
    origin: Vector,
    corner: Vector,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new(
            origin: Vector,
            corner: Vector,
            horizontal: Vector,
            vertical: Vector
        ) -> Camera 
    {
        Camera { origin, corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.corner + u * self.horizontal + v * self.vertical - self.origin
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vector::zero(),
            corner: Vector::new(-2.0, -1.0, -1.0),
            horizontal: Vector::new(4.0, 0.0, 0.0),
            vertical: Vector::new(0.0, 2.0, 0.0)
        }
    }
}