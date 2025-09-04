use rand::prelude::*;

pub fn generate<R: Rng>(rng: &mut R, brown_value: &mut f32) -> Option<f32> {
    *brown_value += rng.random_range(-0.1..0.1);
    *brown_value = brown_value.clamp(-1.0, 1.0);
    Some(*brown_value * 0.3)
}
