pub mod brown;
pub mod pink;
pub mod white;

use crate::types::{Noise, State};
use rodio::Source;
use std::time::Duration;

impl Noise {
    pub fn new(state: State) -> Self {
        Self {
            sample_rate: 44100,
            state,
        }
    }

    pub fn white() -> Self {
        Self::new(State::White)
    }

    pub fn pink() -> Self {
        Self::new(State::Pink {
            values: [0.0; 7],
            counter: 0,
        })
    }

    pub fn brown() -> Self {
        Self::new(State::Brown { value: 0.0 })
    }
}

impl Iterator for Noise {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::rng();

        match &mut self.state {
            State::White => white::generate(&mut rng),
            State::Pink { values, counter } => pink::generate(&mut rng, values, counter),
            State::Brown { value } => brown::generate(&mut rng, value),
        }
    }
}

impl Source for Noise {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
