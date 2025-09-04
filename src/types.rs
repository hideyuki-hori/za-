#[derive(Clone, Copy)]
pub enum Kind {
    White,
    Pink,
    Brown,
}

pub enum State {
    White,
    Pink {
        values: [f32; 7],
        counter: usize,
    },
    Brown {
        value: f32,
    },
}

pub struct Noise {
    pub sample_rate: u32,
    pub state: State,
}
