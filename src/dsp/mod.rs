struct Signal {
    left_phase: f64,
    right_phase: f64,
}

enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
    Pulse,
    Noise,
    Custom,
}
