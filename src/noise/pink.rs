use rand::prelude::*;

pub fn generate<R: Rng>(
    rng: &mut R,
    pink_values: &mut [f32; 7],
    counter: &mut usize,
) -> Option<f32> {
    for i in 0..7 {
        if *counter & (1 << i) == 0 {
            pink_values[i] = rng.random_range(-1.0..1.0);
        }
    }
    *counter += 1;
    let sum: f32 = pink_values.iter().sum();
    Some(sum * 0.1)
}
