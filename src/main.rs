use ray_tracer::{Color, Vec3};

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3 {} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25);
            println!("{}", Vec3::format_color(pixel_color));
        }
    }

    eprintln!("\nDone.")
}
