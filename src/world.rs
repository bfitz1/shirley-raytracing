use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hit::HitRecord;

// Could be more general, but that can until I actually
// need it.
pub struct World {
    pub components: Vec<Sphere>
}

impl World {
    pub fn new(components: Vec<Sphere>) -> World {
        World { components }
    }

    pub fn hit(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.components.iter()
            .filter_map(|item| item.hit(ray, tmin, tmax))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap())
    }
}