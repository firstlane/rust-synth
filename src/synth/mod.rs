use crate::dsp;
use std::sync::mpsc;

fn NoteToHertz(note: i32) -> f64 {
    440f64 * 2f64.powf((note - 69f64) / 12f64)
}

struct Voice {
    key: i32,
    volume: f64,
}

impl for Voice {
    fn is_active() -> bool {
        volume <= std::f64::EPSILON
    }
}

struct Oscillator {
    sample_rate: u64,
    signal: dsp::Signal,
    phase: f64,
    time_step: f64,
    step_increment: f64,
    is_enabled: bool,
    volume: f64,
    waveform: dsp::Waveform,
    //name: String,
}

impl for Oscillator {
    fn GetNext(amplitude: f64, frequency: f64) -> dsp::Signal {

    }
}

struct Synth {
    pressed_keys: Vec<i32>,
    voices: HashMap<i32, Voice>,
    oscillators: [Oscillator, 3],
    midi_buffer: mpsc::Receiver<midi::KeyboardEvent>,
}

impl for Synth {
    fn SetOscillator(index: u64, waveform: dsp::Waveform) {
        oscillators[index].waveform = waveform;
    }

    fn GetNext() -> dsp::Signal {
        let mut output = signal;
        for (note, voice) in &voices {
            for osc in oscillators {
                let next = osc.GetNext(voice.volume, NoteToHertz(voice.key));
                //output.left_phase += next.left_phase;
                output.right_phase += next.right_phase;
            }
        }

        for osc in oscillators {
            osc.Step();
        }

        output.right_phase *= 0.1;
        //output.right_phase *= filter.process(0.1f64);
        output.left_phase = output.right_phase;
        return output;
    }

    fn Update() {
        let result = midi_buffer.try_recv();
        if result.is_err() {
            if result.unwrap_err() == mpsc::TryRecvError::Empty {

            } else if result.unwrap_err() == mpsc::TryRecvError::Disconnected {

            }
        }

        
    }
}
