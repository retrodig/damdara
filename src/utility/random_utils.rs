use rand::Rng;

pub fn random_value(n: u8) -> u8 {
    let mut rng = rand::rng();
    rng.random_range(0..=n)
}
