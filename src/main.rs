use ray_tracer::{Color, Vec3, Ray, Point3, Hittable, Sphere, HittableList, Camera, random, Scatter, Metal, Lambertian, Dielectric, random_range, Material};
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian { albedo: Color::new(0.5, 0.5, 0.5) });
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000., ground_material)));

    let glass: Rc<dyn Material> = Rc::new(Dielectric { ref_idx: 1.5 });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (&center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Rc::new(Lambertian { albedo });
                    world.add(Rc::new(Sphere { center, radius: 0.2, material }))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_range(0., 0.5);
                    let material = Rc::new(Metal { albedo, fuzz });
                    world.add(Rc::new(Sphere { center, radius: 0.2, material }))
                } else {
                    let material = Rc::clone(&glass);
                    world.add(Rc::new(Sphere { center, radius: 0.2, material }))
                }
            }
        }
    }

    let material = Rc::clone(&glass);
    world.add(Rc::new(Sphere { center: Point3::new(0.0, 1.0, 0.0), radius: 1.0, material}));

    let material = Rc::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) });
    world.add(Rc::new(Sphere { center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0, material}));

    let material = Rc::new(Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0});
    world.add(Rc::new(Sphere { center: Point3::new(4.0, 1.0, 0.0), radius: 1.0, material}));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384*2;
    let image_height = (image_width as f64 / aspect_ratio).floor() as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let world = random_scene();

    println!("P3 {} {}", image_width, image_height);
    println!("255");

    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::new(0., 0., 0.);
    let v_up = Vec3::new(0., 1., 0.);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let vfov = 20.;
    let camera = Camera::new(look_from, look_at, v_up, vfov, aspect_ratio, aperture, focus_dist);

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
