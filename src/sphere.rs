use crate::vector::Vector;
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::material::Material;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }

    pub fn hit(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;

        if discriminant > 0.0 {
            let (t1, t2) = (
                (-b - (b*b - a*c).sqrt()) / a,
                (-b + (b*b - a*c).sqrt()) / a,
            );
            if t1 < tmax && t1 > tmin {
                return Some(HitRecord {
                    t: t1,
                    p: ray.travel(t1),
                    normal: (ray.travel(t1) - self.center) / self.radius,
                    material: self.material,
                });
            } else if t2 < tmax && t2 > tmax {
                return Some(HitRecord {
                    t: t2,
                    p: ray.travel(t2),
                    normal: (ray.travel(t2) - self.center) / self.radius,
                    material: self.material,
                });
            }
        }
        None
    }
}