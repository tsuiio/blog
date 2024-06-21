use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = thread_rng();
    
    Alphanumeric.sample_string(&mut rng, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_string() {
        let length = 10;

        let random_string = generate_random_string(length);

        assert_eq!(random_string.len(), length);
        assert!(is_random(&random_string));
    }

    fn is_random(s: &str) -> bool {
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();
        chars.any(|c| c != first_char)
    }
}
