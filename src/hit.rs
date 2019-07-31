use crate::vector::Vector;
use crate::material::Material;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
    pub material: Material,
}