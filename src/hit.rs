use crate::vector::Vector;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { list }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest = tmax;
        for item in self.list.iter() {
            if let Some(record) = item.hit(ray, tmin, closest) {
                closest = record.t;
                result = Some(record);
            }
        }
        result
    }
}