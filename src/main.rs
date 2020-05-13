use ray_tracer::{Color, Vec3, Ray, Point3, Hittable, Sphere, HittableList, Camera, random, Scatter, Metal, Lambertian};
use std::f64;
use std::rc::Rc;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i16) -> Color {
    if depth <= 0 {
        Color::new(0., 0., 0.)
    } else {
        match world.hit(r, 0.001, f64::INFINITY) {
            Some(rec) => {
                match rec.material.scatter(r, &rec) {
                    Some(Scatter { scattered, attenuation }) =>
                        attenuation * ray_color(&scattered, world, depth - 1),
                    None => Color::new(0., 0., 0.)
                }
            }
            None => {
                // Blue/white gradient
                let unit_direction = r.direction().unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio).floor() as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("P3 {} {}", image_width, image_height);
    println!("255");

    let camera = Camera::new();
    let mut world = HittableList::new();
    world.add(Rc::new(
        Sphere {
            center: Point3::new(0., 0., -1.),
            radius: 0.5,
            material: Rc::new(Lambertian { albedo: Color::new(0.7, 0.3, 0.3) }),
        }));
    world.add(Rc::new(
        Sphere {
            center: Point3::new(0., -100.5, -1.),
            radius: 100.,
            material: Rc::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.) }),
        }));
    world.add(Rc::new(
        Sphere {
            center: Point3::new(1., 0., -1.),
            radius: 0.5,
            material: Rc::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1. }),
        }));
    world.add(Rc::new(
        Sphere {
            center: Point3::new(-1., 0., -1.),
            radius: 0.5,
            material: Rc::new(Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0. }),
        }));

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let pixel_color = (0..samples_per_pixel)
                .map(|_| {
                    let u = (i as f64 + random()) / (image_width - 1) as f64;
                    let v = (j as f64 + random()) / (image_height - 1) as f64;
                    camera.get_ray(u, v)
                })
                .fold(Color::new(0., 0., 0.), |pixel_color, ray| {
                    pixel_color + ray_color(&ray, &world, max_depth)
                });
            println!("{}", Vec3::format_color(pixel_color, samples_per_pixel));
        }
    }

    eprintln!("\nDone.")
}
