use std::fmt;

#[derive(Debug)]
pub enum State {
    White,
    Pink { values: [f32; 7], counter: usize },
    Brown { value: f32 },
}

pub struct Noise {
    pub sample_rate: u32,
    pub state: State,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::White => write!(f, "White"),
            State::Pink { .. } => write!(f, "Pink"),
            State::Brown { .. } => write!(f, "Brown"),
        }
    }
}
