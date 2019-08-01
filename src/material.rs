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

fn refract(v: Vector, n: Vector, ni_over_nt: f64) -> Option<Vector> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt*n) - discriminant.sqrt()*n)
    } else {
        None
    }
}

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    pub fn scatter(&self, ray: Ray, record: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = reflect(ray.direction, record.normal);
        let attenuation = Vector::ones();

        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(record.normal) > 0.0 {
            (
                -record.normal,
                self.refractive_index,
                self.refractive_index * ray.direction.dot(record.normal) / ray.direction.length(),
            )
        } else {
            (
                record.normal,
                1.0 / self.refractive_index,
                -ray.direction.dot(record.normal) / ray.direction.length(),
            )
        };

        let scattered = match refract(ray.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                if rng.gen::<f64>() < schlick(cosine, self.refractive_index) {
                    Ray::new(record.p, reflected)
                } else {
                    Ray::new(record.p, refracted)
                }
            },
            None => Ray::new(record.p, reflected),
        };

        Some(Scatter { attenuation, scattered })
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn lambertian(albedo: Vector) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }

    pub fn metal(albedo: Vector, fuzz: f64) -> Material {
        let fuzz = fuzz.min(1.0);
        Material::Metal(Metal { albedo, fuzz })
    }

    pub fn dielectric(refractive_index: f64) -> Material {
        Material::Dielectric(Dielectric { refractive_index })
    }

    pub fn scatter(&self, ray: Ray, record: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, record, rng),
            Material::Metal(m) => m.scatter(ray, record, rng),
            Material::Dielectric(d) => d.scatter(ray, record, rng),
        }
    }
}