use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = thread_rng();

    Alphanumeric.sample_string(&mut rng, length)
}
