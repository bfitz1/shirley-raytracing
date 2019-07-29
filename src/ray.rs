use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn travel(&self, t: f64) -> Vector {
        self.origin + t * self.direction
    }
}