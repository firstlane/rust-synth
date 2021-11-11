pub struct Signal {
    pub left_phase: f64,
    pub right_phase: f64,
}

pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
    Pulse,
    Noise,
    Custom,
}
