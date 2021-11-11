use crate::dsp;
use crate::midi;

use std::sync::mpsc;
use std::collections::HashMap;

fn NoteToHertz(note: i32) -> f64 {
    440f64 * 2f64.powf((note - 69f64) / 12f64)
}

struct Voice {
    key: i32,
    volume: f64,
}

impl Voice {
    fn is_active(&mut self) -> bool {
        self.volume <= std::f64::EPSILON
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

impl Oscillator {
    fn GetNext(&mut self, amplitude: f64, frequency: f64) -> dsp::Signal {
        self.signal.left_phase = amplitude * f64::sin(2f64 * std::f64::consts::PI * self.time_step + self.phase);
        self.signal.right_phase = self.signal.left_phase;
        return self.signal;
    }

    fn Step(&mut self) {
        self.time_step += self.step_increment;

        if self.time_step > (std::f64::consts::PI * 2f64) {
            self.time_step = self.time_step - (std::f64::consts::PI * 2f64)
        }
    }
}

struct Synth {
    pressed_keys: Vec<i32>,
    voices: HashMap<i32, Voice>,
    oscillators: [Oscillator; 3],
    midi_buffer: mpsc::Receiver<midi::KeyboardEvent>,
}

impl Synth {
    fn SetOscillator(&mut self, index: usize, waveform: dsp::Waveform) {
        self.oscillators[index].waveform = waveform;
    }

    fn GetNext(&mut self) -> dsp::Signal {
        let mut output = dsp::Signal{
            left_phase: 0f64,
            right_phase: 0f64,
        };

        for (note, voice) in &self.voices {
            for osc in self.oscillators {
                let next = osc.GetNext(voice.volume, NoteToHertz(voice.key));
                output.right_phase += next.right_phase;
            }
        }

        for osc in self.oscillators {
            osc.Step();
        }

        output.right_phase *= 0.1;
        //output.right_phase *= filter.process(0.1f64);
        output.left_phase = output.right_phase;
        return output;
    }

    fn Update(&mut self) {
        let result = self.midi_buffer.try_recv();
        if result.is_err() {
            if result.unwrap_err() == mpsc::TryRecvError::Empty {

            } else if result.unwrap_err() == mpsc::TryRecvError::Disconnected {

            }
        }

        
    }
}
