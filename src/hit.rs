use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
}