use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// fvov: vertical field-of-view in degrees
    pub fn new(look_from: Point3,
               look_at: Point3,
               v_up: Vec3, vfov: f64,
               aspect_ratio: f64,
               aperture: f64,
               focus_dist: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);
        // eprintln!("height:{} width:{} u:{:?} v:{:?} w:{:?}", viewport_height, viewport_width, u, v, w);

        let origin = look_from.clone();
        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        let lower_left_corner = &origin - &horizontal / 2. - &vertical / 2. - focus_dist * &w;
        let lens_radius = aperture / 2.;
        Camera { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = &self.u * rd.x() + &self.v * rd.y();
        let origin = &self.origin + &offset;
        let direction = &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - offset;
        Ray::new(origin, direction)
    }
}