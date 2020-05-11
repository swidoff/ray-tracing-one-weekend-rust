use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let lower_left_corner = Point3::new(-2., -1., -1.);
        let horizontal = Vec3::new(4., 0., 0.);
        let vertical = Vec3::new(0., 2.0, 0.);
        let origin = Point3::new(0., 0., 0.);
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin.clone(), &self.lower_left_corner + &(u * &self.horizontal) + v * &self.vertical)
    }
}