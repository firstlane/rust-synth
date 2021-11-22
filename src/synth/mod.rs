use crate::dsp;
use crate::midi;

use std::sync::mpsc;
use std::collections::HashMap;

fn NoteToHertz(note: i32) -> f64 {
    440f64 * 2f64.powf((note as f64 - 69f64) / 12f64)
}

const KEYS: &str = "ZSXCFVGBNJMK";

pub struct Voice {
    pub key: i32,
    pub volume: f64,
}

impl Voice {
    fn is_active(&mut self) -> bool {
        self.volume <= std::f64::EPSILON
    }
}

#[derive(Copy, Clone)]
pub struct Oscillator {
    pub sample_rate: f64,
    pub signal: dsp::Signal,
    pub phase: f64,
    pub time_step: f64,
    pub step_increment: f64,
    pub is_enabled: bool,
    pub volume: f64,    // TODO: this guy isn't being used anywhere right now
    pub waveform: dsp::Waveform,
    //name: String,
}

impl Oscillator {
    pub fn new(sample_rate: f64, waveform: dsp::Waveform) -> Self {
        Oscillator{
            sample_rate: sample_rate,
            signal: dsp::Signal{
                left_phase: 0f64,
                right_phase: 0f64,
            },
            phase: 0f64,
            time_step: 0f64,
            step_increment: 1f64 / sample_rate,
            is_enabled: true,
            volume: 0f64,
            waveform: waveform,
        }
    }

    pub fn GetNext(&mut self, amplitude: f64, frequency: f64) -> dsp::Signal {
        self.signal.left_phase = amplitude * f64::sin(2f64 * std::f64::consts::PI * self.time_step + self.phase);
        self.signal.right_phase = self.signal.left_phase;
        return self.signal;
    }

    pub fn Step(&mut self) {
        self.time_step += self.step_increment;

        if self.time_step > (std::f64::consts::PI * 2f64) {
            self.time_step = self.time_step - (std::f64::consts::PI * 2f64)
        }
    }
}

pub struct Synth {
    pub pressed_keys: Vec<i32>,
    pub voices: HashMap<i32, Voice>, // TODO: why do I have the key here for the note? Voice already has a note.
    pub oscillators: [Oscillator; 3],
    pub midi_buffer: mpsc::Receiver<midi::KeyboardEvent>,
}

impl Synth {
    pub fn new(midi_rx: mpsc::Receiver<midi::KeyboardEvent>, sample_rate: f64, waveform: dsp::Waveform) -> Self {
        Synth{
            pressed_keys: Vec::new(),
            voices: HashMap::new(),
            oscillators: [
                Oscillator::new(sample_rate, waveform);
                3
            ],
            midi_buffer: midi_rx,
        }
    }

    pub fn SetOscillator(&mut self, index: usize, waveform: dsp::Waveform) {
        self.oscillators[index].waveform = waveform;
    }

    pub fn GetNext(&mut self) -> dsp::Signal {
        let mut output = dsp::Signal{
            left_phase: 0f64,
            right_phase: 0f64,
        };

        for (_note, voice) in &self.voices {
            for osc in self.oscillators.iter_mut() {
                let next = osc.GetNext(voice.volume, NoteToHertz(voice.key));
                output.right_phase += next.right_phase;
            }
        }

        for osc in self.oscillators.iter_mut() {
            osc.Step();
        }

        output.right_phase *= 0.1;
        //output.right_phase *= filter.process(0.1f64);
        output.left_phase = output.right_phase;
        //println!("output: {}, {}", output.left_phase, output.right_phase);
        return output;
    }

    pub fn Update(&mut self) {
        let result = self.midi_buffer.try_recv();
        if result.is_err() {
            //if result.unwrap_err() == mpsc::TryRecvError::Empty {

            //} else if result.unwrap_err() == mpsc::TryRecvError::Disconnected {

            //}
            //let err = result.unwrap_err();
        }
        else {
            let midi_event = result.unwrap();

            //println!("Received from midi buffer");

            if midi_event.on {
                //println!("On");
                self.voices.insert(midi_event.key as i32, Voice{ key: midi_event.key as i32, volume: 100f64 / 127f64 });
            }
            else {
                //println!("Off");
                self.voices.remove(&(midi_event.key as i32));
            }
        }
    }
}
