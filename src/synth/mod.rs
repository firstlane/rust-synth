use crate::dsp;
use crate::midi;

use maplit::hashmap;

use std::sync::mpsc;
use std::collections::HashMap;
use std::array::IntoIter;
use std::iter::FromIterator;

fn NoteToHertz(note: i32) -> f64 {
    440f64 * 2f64.powf((note as f64 - 69f64) / 12f64)
}

//const KEYS: &device_query::Keycode = "OPZSXCFVGBNJMK";
// const KEY_MAP: HashMap::<device_query::Keycode, i32>::from_iter(IntoIter::new([
//     (device_query::Keycode::Z, 0x39i32),
//     (device_query::Keycode::X, 0x39i32 + 1),
//     (device_query::Keycode::C, 0x39i32 + 2),
//     (device_query::Keycode::V, 0x39i32 + 3),
//     (device_query::Keycode::B, 0x39i32 + 4),
//     (device_query::Keycode::N, 0x39i32 + 5),
//     (device_query::Keycode::M, 0x39i32 + 6),
//     ]));
// const KEY_MAP: HashMap::from([
//     (device_query::Keycode::Z, i32); 5]),
//     (device_query::Keycode::X, 0x39i32 + 1),
//     (device_query::Keycode::C, 0x39i32 + 2),
//     (device_query::Keycode::V, 0x39i32 + 3),
//     (device_query::Keycode::B, 0x39i32 + 4),
//     (device_query::Keycode::N, 0x39i32 + 5),
//     (device_query::Keycode::M, 0x39i32 + 6),
//     ]);
// const KEY_MAP: maplit::hashmap!{
//     device_query::Keycode::Z => 0x39i32,
//     device_query::Keycode::X => 0x39i32 + 1,
//     device_query::Keycode::C => 0x39i32 + 2,
//     device_query::Keycode::V => 0x39i32 + 3,
//     device_query::Keycode::B => 0x39i32 + 4,
//     device_query::Keycode::N => 0x39i32 + 5,
//     device_query::Keycode::M => 0x39i32 + 6,
// };

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
        self.signal.left_phase = amplitude * f64::sin(2f64 * std::f64::consts::PI * frequency * self.time_step + self.phase);
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
}

impl Synth {
    pub fn new(sample_rate: f64, waveform: dsp::Waveform) -> Self {
        Synth{
            pressed_keys: Vec::new(),
            voices: HashMap::new(),
            oscillators: [
                Oscillator::new(sample_rate, waveform);
                3
            ],
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

    pub fn Update(&mut self, midi_event: midi::KeyboardEvent) {
        // let result = self.midi_buffer.try_recv();
        // if result.is_err() {
        //     //if result.unwrap_err() == mpsc::TryRecvError::Empty {

        //     //} else if result.unwrap_err() == mpsc::TryRecvError::Disconnected {

        //     //}
        //     //let err = result.unwrap_err();
        // }
        // else {
            //let midi_event = result.unwrap();

            //println!("Received from midi buffer");

            let mut key = 0x39i32;

            match midi_event.key {
                device_query::Keycode::Z => {
                },
                device_query::Keycode::X => {
                    key = key + 1;
                },
                device_query::Keycode::C => {
                    key = key + 2;
                },
                device_query::Keycode::V => {
                    key = key + 3;
                },
                device_query::Keycode::B => {
                    key = key + 4;
                },
                device_query::Keycode::N => {
                    key = key + 5;
                },
                device_query::Keycode::M => {
                    key = key + 6;
                },
                _ => {

                }
            }

            if midi_event.on {
                //println!("On");
                self.voices.insert(key, Voice{ key: key as i32, volume: 100f64 / 127f64 });
            }
            else {
                //println!("Off");
                self.voices.remove(&key);
            }
        //}
    }
}
