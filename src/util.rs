use rand;
use rand::Rng;

/// Generate a random f64 between [0., 1.).
pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0., 1.)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}