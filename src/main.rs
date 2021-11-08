extern crate anyhow;
extern crate clap;
extern crate cpal;
extern crate device_query;

mod midi;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat};
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

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

    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f32;
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let (sender, receiver): (SyncSender<dsp::KeyboardEvent, Receiver<dsp::KeyboardEvent>) = std::sync::mpsc::sync_channel(1024);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_secs(3));

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
