use ray_tracer::{Color, Vec3, Ray, Point3};

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = *ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let half_b = ray.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0., 0., -1.), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0., 0., -1.)).unit_vector();
        0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1.)
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio).floor() as i32;

    println!("P3 {} {}", image_width, image_height);
    println!("255");
    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2.25, 0.);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., 1.0);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let pixel_color = ray_color(&r);
            println!("{}", Vec3::format_color(pixel_color));
        }
    }

    eprintln!("\nDone.")
}
