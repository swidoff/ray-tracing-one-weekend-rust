use ray_tracer::{Color, Vec3, Ray, Point3, Hittable, Sphere, HittableList};
use std::f64;
use std::rc::Rc;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    match world.hit(r, 0., f64::INFINITY) {
        Some(rec) =>
        // Normal vector coloring.
            0.5 * (rec.normal + Color::new(1., 1., 1.)),
        None => {
            // Blue/white gradient
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio).floor() as i32;

    println!("P3 {} {}", image_width, image_height);
    println!("255");

    let lower_left_corner = Point3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2.0, 0.);
    let origin = Point3::new(0., 0., 0.);

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let pixel_color = ray_color(&r, &world);
            println!("{}", Vec3::format_color(pixel_color));
        }
    }

    eprintln!("\nDone.")
}
