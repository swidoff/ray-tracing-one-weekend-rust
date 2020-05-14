use crate::ray::Ray;
use crate::vec3::{Vec3, Color};
use crate::hittable::HitRecord;
use crate::util::random;

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = &rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p.clone(), scatter_direction);
        let attenuation = self.albedo.clone();
        Some(Scatter { scattered, attenuation })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&ray.direction().unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p.clone(), reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo.clone();
        Some(Scatter { scattered, attenuation })
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Color::new(1., 1., 1.);
        let etai_over_etat = if rec.front_face { 1. / self.ref_idx} else { self.ref_idx };

        let unit_direction = Vec3::unit_vector(ray.direction());
        let cos_theta = (-&unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();
        if etai_over_etat*sin_theta > 1. ||
            schlick(cos_theta, etai_over_etat) > random() {
            let reflected = reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new(rec.p.clone(), reflected);
            Some(Scatter { scattered, attenuation})
        } else {
            let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
            let scattered = Ray::new(rec.p.clone(), refracted);
            Some(Scatter { scattered, attenuation})
        }
    }
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n);
    let r_out_parallel: Vec3 = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp: Vec3 = -(1.0 - r_out_parallel.length_squared()).sqrt() * n;
    r_out_parallel + r_out_perp
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}