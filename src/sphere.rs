use crate::vector::Vector;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hitable};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < tmax && temp > tmin {
                return Some(HitRecord {
                    t: temp,
                    p: ray.travel(temp),
                    normal: (ray.travel(temp) - self.center) / self.radius
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < tmax && temp > tmin {
                return Some(HitRecord {
                    t: temp,
                    p: ray.travel(temp),
                    normal: (ray.travel(temp) - self.center) / self.radius
                });
            }
        }
        None
    }
}