#![feature(clamp)]

mod vec3;
mod ray;
mod hittable;
mod camera;
mod util;
mod material;

pub use crate::vec3::{Vec3, Color, Point3};
pub use crate::ray::Ray;
pub use crate::hittable::{Hittable, Sphere, HittableList};
pub use crate::material::{Lambertian, Material, Metal, Scatter, Dielectric};
pub use crate::camera::Camera;
pub use crate::util::{random, random_range};