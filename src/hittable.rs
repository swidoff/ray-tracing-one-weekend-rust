use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use std::rc::Rc;
use crate::material::Material;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    fn from_outward_normal<F>(r: &Ray, t: f64, material: &Rc<dyn Material>, outward_normal_fn: F) -> HitRecord
        where F: FnOnce(&Point3) -> Vec3 {
        let p = r.at(t);
        let outward_normal = outward_normal_fn(&p);
        let front_face = r.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        HitRecord { p, normal, t, front_face, material: Rc::clone(material) }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere { center, radius, material: Rc::clone(&material) }
    }

    pub fn outward_normal(&self, p: &Point3) -> Vec3 {
        (p - &self.center) / self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().dot(ray.direction());
        let half_b = ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let root = discriminant.sqrt();
            let solutions = [(-half_b - root) / a, (-half_b + root) / a];
            (0..2).map(|i| solutions[i])
                .find(|t| *t < t_max && *t > t_min)
                .map(|t| {
                    HitRecord::from_outward_normal(&ray, t, &self.material, |p| self.outward_normal(p))
                })
        }
    }
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn from_hittable(hittable: Rc<dyn Hittable>) -> HittableList {
        let mut ls = HittableList::new();
        ls.objects.push(Rc::clone(&hittable));
        ls
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.objects.push(Rc::clone(&hittable));
    }
}

impl Hittable for HittableList {
    /// Returns the hit of the closest object, if there is a hit.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects.iter()
            .fold(None, |prev, object| {
                let t_closest = prev.iter().find_map(|r| Some(r.t)).unwrap_or(t_max);
                let rec = object.hit(ray, t_min, t_closest);
                rec.or(prev)
            })
    }
}