#![feature(clamp)]

mod vec3;
mod ray;
mod hittable;
mod camera;
mod util;

pub use crate::vec3::{Vec3, Color, Point3};
pub use crate::ray::Ray;
pub use crate::hittable::{Hittable, Sphere, HittableList};
pub use crate::camera::Camera;
pub use crate::util::random;