use std::fmt;
use std::iter;
use std::ops::{Index, IndexMut};
use auto_ops::*;
use crate::util::{random, random_range};
use std::f64::consts::PI;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Color { e: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random(), random(), random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_range(min, max), random_range(min, max), random_range(min, max))
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        iter::repeat_with(|| Vec3::random_range(-1., 1.))
            .find(|v| v.length_squared() < 1.)
            .unwrap()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        iter::repeat_with(|| {
            let x = random_range(-1., 1.);
            let y = random_range(-1., 1.);
            Vec3::new(x, y, 0.)
        })
            .find(|v| v.length_squared() < 1.)
            .unwrap()
    }

    pub fn random_unit_vector() -> Vec3 {
        let a = random_range(0., 2. * PI);
        let z = random_range(-1., 1.);
        let r = (1. - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(&self, rhs: &Self) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ]
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let len = self.length();
        self / len
    }

    pub fn format_color(color: Color, samples: u16) -> String {
        let scale = 1. / samples as f64;
        let r = (color.x() * scale).sqrt();
        let g = (color.y() * scale).sqrt();
        let b = (color.z() * scale).sqrt();

        let ir = (256. * r.clamp(0., 0.999)) as u8;
        let ig = (256. * g.clamp(0., 0.999)) as u8;
        let ib = (256. * b.clamp(0., 0.999)) as u8;
        format!("{} {} {}", ir, ig, ib)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl_op_ex!(- |a: &Vec3| -> Vec3 {
    Vec3 { e: [-a.e[0], -a.e[1], -a.e[2]] }
});

impl_op_ex!(+= |a: &mut Vec3, b: Vec3| {
    a.e[0] += b.e[0];
    a.e[1] += b.e[1];
    a.e[2] += b.e[2];
});

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 { e: [a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2]] }
});

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 { e: [a.e[0] - b.e[0], a.e[1] - b.e[1], a.e[2] - b.e[2]] }
});

impl_op_ex!(*= |a: &mut Vec3, b: f64| {
    a.e[0] *= b;
    a.e[1] *= b;
    a.e[2] *= b;
});

impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 { e: [a.e[0] * b.e[0], a.e[1] * b.e[1], a.e[2] * b.e[2]] }
});

impl_op_ex_commutative!(* |a: &Vec3, b: f64| -> Vec3 {
    Vec3 { e: [a.e[0] * b, a.e[1] * b, a.e[2] * b] }
});

impl_op_ex!(/= |a: &mut Vec3, b: f64| {
    *a *= 1.0 / b
});

impl_op_ex!(/ |a: &Vec3, b: &f64| -> Vec3 {
    a * (1.0 / b)
});

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}