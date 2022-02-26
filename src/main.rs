extern crate anyhow;
extern crate clap;
extern crate cpal;
extern crate device_query;
extern crate maplit;

mod midi;
mod dsp;
mod synth;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat};
use device_query::{DeviceEvents, DeviceQuery, DeviceState, MouseState, Keycode};

use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time;

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

    let synth = Arc::new(Mutex::new(synth::Synth::new(sample_rate as f64, dsp::Waveform::Sine)));
    let synth_sampler = Arc::clone(&synth);

    let mut next_value = move |synth: &mut MutexGuard<synth::Synth>| {
        //sample_clock = (sample_clock + 1.0) % sample_rate;
        //(sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        //let mut synth = synth_sampler.lock().unwrap();
        synth.get_next().left_phase as f32
    };

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let mut synth = synth_sampler.lock().unwrap();
            write_data(data, channels, &mut next_value, &mut synth)
        },
        err_fn,
    )?;

    stream.play()?;

    loop {
        let result = midi_rx.try_recv();
        if result.is_err() {
            //if result.unwrap_err() == mpsc::TryRecvError::Empty {

            //} else if result.unwrap_err() == mpsc::TryRecvError::Disconnected {

            //}
            //let err = result.unwrap_err();
        }
        else {
            let midi_event = result.unwrap();

            println!("Received from midi buffer");
            let mut synth_update = synth.lock().unwrap();
            synth_update.update(midi_event);
        }
    }

    Ok(())
}

fn synth_get_next(synth: &mut MutexGuard<synth::Synth>) -> f32 {
    synth.get_next().left_phase as f32
}

fn write_data<T: Sample>(data: &mut [T], channels: usize, next_sample: &mut dyn FnMut(&mut MutexGuard<synth::Synth>) -> f32, synth: &mut MutexGuard<synth::Synth>) {
    for frame in data.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample(synth));
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn midi_listen(midi_sender: mpsc::SyncSender<midi::KeyboardEvent>) {
    let device_state = DeviceState::new();
    let key_up_sender = midi_sender.clone();
    let _guard = device_state.on_key_down(move |key| {
        let result = midi_sender.send(midi::KeyboardEvent{
            key: *key,
            on: true,
        });
    });
    let _guard = device_state.on_key_up(move |key| {
        let result = key_up_sender.send(midi::KeyboardEvent{
            key: *key,
            on: false,
        });
    });

    loop {
    }
}
