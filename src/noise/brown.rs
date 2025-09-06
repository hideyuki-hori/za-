use rand::prelude::*;

pub fn generate<R: Rng>(rng: &mut R, value: &mut f32) -> Option<f32> {
    *value += rng.random_range(-0.1..0.1);
    *value = value.clamp(-1.0, 1.0);
    Some(*value * 0.3)
}
