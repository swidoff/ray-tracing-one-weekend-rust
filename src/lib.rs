#![feature(clamp)]

mod vec3;
mod ray;
mod hittable;
mod camera;

pub use crate::vec3::{Vec3, Color, Point3};
pub use crate::ray::Ray;
pub use crate::hittable::{Hittable, Sphere, HittableList};
pub use crate::camera::Camera;

use rand;
use rand::Rng;

/// Generate a random f64 between [0., 1.).
pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0., 1.)
}