use rand::prelude::*;

pub fn generate<R: Rng>(rng: &mut R) -> Option<f32> {
    Some(rng.random_range(-0.5..0.5))
}
