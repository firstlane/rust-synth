extern crate anyhow;
extern crate clap;
extern crate cpal;
extern crate device_query;

mod midi;
mod dsp;
mod synth;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat};
use device_query::{DeviceEvents, DeviceQuery, DeviceState, MouseState, Keycode};

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host.default_output_device()
                     .expect("no output device available");

//    let mut supported_configs_range = device.supported_output_configs()
//                                            .expect("error while querying configs");
//    let supported_config = supported_configs_range.next()
//        .expect("no supported config?!")
//        .with_max_sample_rate();
//     let sample_format = supported_config.sample_format();
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    }
}

pub fn run<T: Sample>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
{
    let err_fn = |err| eprintln!("an error occurrred on the output audio stream: {}", err);

    let (midi_tx, midi_rx) = mpsc::sync_channel::<midi::KeyboardEvent>(1024);

    let midi_listen_thread = std::thread::spawn(move || {
        midi_listen(midi_tx);
    });

    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    let synth = Arc::new(Mutex::new(synth::Synth::new(midi_rx, sample_rate as f64, dsp::Waveform::Sine)));
    let synth_sampler = Arc::clone(&synth);

    let mut next_value = move || {
        //sample_clock = (sample_clock + 1.0) % sample_rate;
        //(sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        let mut synth = synth_sampler.lock().unwrap();
        synth.GetNext().left_phase as f32
    };

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;

    stream.play()?;

    //std::thread::sleep(std::time::Duration::from_secs(3));

    loop {
        let mut synth_update = synth.lock().unwrap();
        synth_update.Update();
    }

    Ok(())
}

fn write_data<T: Sample>(data: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32) {
    for frame in data.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn midi_listen(midi_sender: mpsc::SyncSender<midi::KeyboardEvent>) {
    let device_state = DeviceState::new();
    let key_up_sender = midi_sender.clone();
    let _guard = device_state.on_key_down(move |key| {
        println!("On key down: {}", key);
        let result = midi_sender.send(midi::KeyboardEvent{
            key: *key,
            on: true,
        });
    });
    let _guard = device_state.on_key_up(move |key| {
        println!("On key up: {}", key);
        let result = key_up_sender.send(midi::KeyboardEvent{
            key: *key,
            on: false,
        });
    });

    loop {
    }
}
