use crate::vector::Vector;
use crate::ray::Ray;
use crate::hit::HitRecord;
use rand::{Rng, rngs::ThreadRng};

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector {
    loop {
        let p = 2.0 * Vector::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            rng.gen::<f64>(),
        ) - Vector::ones();
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

fn reflect(v: Vector, n: Vector) -> Vector {
    v - 2.0 * v.dot(n) * n
}

#[derive(Copy, Clone)]
pub struct Scatter {
    pub attenuation: Vector,
    pub scattered: Ray,
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vector,
}

impl Lambertian {
    pub fn scatter(&self, _: Ray, record: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        let target = record.p + record.normal + random_in_unit_sphere(rng);
        let attenuation = self.albedo;
        let scattered = Ray::new(record.p, target - record.p);

        Some(Scatter { attenuation, scattered })
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vector,
    pub fuzz: f64,
}

impl Metal {
    pub fn scatter(&self, ray: Ray, record: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = reflect(ray.direction.unit(), record.normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(record.p, reflected + self.fuzz*random_in_unit_sphere(rng));

        if scattered.direction.dot(record.normal) > 0.0 {
            Some(Scatter { attenuation, scattered })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn lambertian(albedo: Vector) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }

    pub fn metal(albedo: Vector, fuzz: f64) -> Material {
        let fuzz = fuzz.min(1.0);
        Material::Metal(Metal { albedo, fuzz })
    }

    pub fn scatter(&self, ray: Ray, record: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, record, rng),
            Material::Metal(m) => m.scatter(ray, record, rng),
        }
    }
}