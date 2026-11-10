use crate::dsp::{Chain, Params};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device, Stream, StreamConfig};
use ringbuf::traits::{Consumer, Observer, Producer, Split};
use ringbuf::{HeapCons, HeapRb};
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub struct Engine {
    _input: Stream,
    _output: Stream,
    pub sample_rate: u32,
    pub tap: HeapCons<f32>,
}

fn device_name(device: &Device) -> Option<String> {
    device.description().ok().map(|d| d.name().to_string())
}

pub fn input_names() -> Vec<String> {
    cpal::default_host()
        .input_devices()
        .map(|list| list.filter_map(|d| device_name(&d)).collect())
        .unwrap_or_default()
}

pub fn output_names() -> Vec<String> {
    cpal::default_host()
        .output_devices()
        .map(|list| list.filter_map(|d| device_name(&d)).collect())
        .unwrap_or_default()
}

pub fn default_input_name() -> Option<String> {
    cpal::default_host()
        .default_input_device()
        .and_then(|d| device_name(&d))
}

pub fn default_output_name() -> Option<String> {
    cpal::default_host()
        .default_output_device()
        .and_then(|d| device_name(&d))
}

fn find_input(name: &str) -> Option<Device> {
    cpal::default_host()
        .input_devices()
        .ok()?
        .find(|d| device_name(d).as_deref() == Some(name))
}

fn find_output(name: &str) -> Option<Device> {
    cpal::default_host()
        .output_devices()
        .ok()?
        .find(|d| device_name(d).as_deref() == Some(name))
}

fn open(
    input: &Device,
    output: &Device,
    in_channels: u16,
    out_channels: u16,
    sample_rate: u32,
    buffer_size: BufferSize,
    params: &Arc<Params>,
) -> Result<Engine, String> {
    let in_config = StreamConfig {
        channels: in_channels,
        sample_rate,
        buffer_size,
    };
    let out_config = StreamConfig {
        channels: out_channels,
        sample_rate,
        buffer_size,
    };

    let ring = HeapRb::<f32>::new(sample_rate as usize);
    let (mut producer, mut consumer) = ring.split();
    let (mut tap_producer, tap) = HeapRb::<f32>::new(sample_rate as usize).split();

    let in_params = params.clone();
    let in_ch = in_channels as usize;
    let input_stream = input
        .build_input_stream(
            in_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let channel = in_params
                    .input_channel
                    .load(Ordering::Relaxed)
                    .min(in_ch - 1);
                let mut peak = 0.0f32;
                for frame in data.chunks(in_ch) {
                    let sample = frame[channel];
                    peak = peak.max(sample.abs());
                    let _ = producer.try_push(sample);
                    let _ = tap_producer.try_push(sample);
                }
                in_params
                    .in_level
                    .set(peak.max(in_params.in_level.get() * 0.9));
            },
            |_| {},
            None,
        )
        .map_err(|e| format!("Input: {e}"))?;

    let out_params = params.clone();
    let out_ch = out_channels as usize;
    let mut chain = Chain::new(sample_rate);
    let mut settings = crate::dsp::Settings::clean();
    let max_backlog = (sample_rate as usize / 100).max(256);
    let output_stream = output
        .build_output_stream(
            out_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                if let Some(latest) = out_params.try_load() {
                    settings = latest;
                }
                let frames = data.len() / out_ch;
                let waiting = consumer.occupied_len();
                if waiting > frames + max_backlog {
                    consumer.skip(waiting - frames - max_backlog / 2);
                }
                let mut peak = 0.0f32;
                chain.reduction = 1.0;
                for frame in data.chunks_mut(out_ch) {
                    let dry = consumer.try_pop().unwrap_or(0.0);
                    let wet = chain.process(dry, &settings);
                    peak = peak.max(wet.abs());
                    for sample in frame {
                        *sample = wet;
                    }
                }
                let held = out_params.reduction.get();
                out_params
                    .reduction
                    .set(chain.reduction.min(held + (1.0 - held) * 0.1));
                out_params
                    .out_level
                    .set(peak.max(out_params.out_level.get() * 0.9));
            },
            |_| {},
            None,
        )
        .map_err(|e| format!("Output: {e}"))?;

    input_stream.play().map_err(|e| format!("Input: {e}"))?;
    output_stream.play().map_err(|e| format!("Output: {e}"))?;

    Ok(Engine {
        _input: input_stream,
        _output: output_stream,
        sample_rate,
        tap,
    })
}

pub fn start(input_name: &str, output_name: &str, params: &Arc<Params>) -> Result<Engine, String> {
    let input = find_input(input_name).ok_or("Input device not found")?;
    let output = find_output(output_name).ok_or("Output device not found")?;
    let in_default = input
        .default_input_config()
        .map_err(|e| format!("Input: {e}"))?;
    let out_default = output
        .default_output_config()
        .map_err(|e| format!("Output: {e}"))?;
    let in_channels = in_default.channels();
    let out_channels = out_default.channels();
    params
        .input_channels
        .store(in_channels as usize, Ordering::Relaxed);

    let mut rates = vec![out_default.sample_rate()];
    if in_default.sample_rate() != out_default.sample_rate() {
        rates.push(in_default.sample_rate());
    }

    let mut last_error = String::new();
    for rate in rates {
        for buffer_size in [
            BufferSize::Fixed(128),
            BufferSize::Fixed(256),
            BufferSize::Default,
        ] {
            match open(
                &input,
                &output,
                in_channels,
                out_channels,
                rate,
                buffer_size,
                params,
            ) {
                Ok(engine) => return Ok(engine),
                Err(e) => last_error = e,
            }
        }
    }
    Err(last_error)
}
