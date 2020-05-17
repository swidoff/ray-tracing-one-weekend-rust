use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// fvov: vertical field-of-view in degrees
    pub fn new(look_from: Point3, look_at: Point3, v_up: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);
        // eprintln!("height:{} width:{} u:{:?} v:{:?} w:{:?}", viewport_height, viewport_width, u, v, w);

        let origin = look_from.clone();
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = &origin - &horizontal / 2. - &vertical / 2. - w;
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = self.origin.clone();
        let direction = &self.lower_left_corner + &(u * &self.horizontal) + v * &self.vertical - &origin;
        Ray::new(origin, direction)
    }
}