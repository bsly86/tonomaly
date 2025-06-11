/// handle the actual audio output
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig};
use std::sync::{Arc, Mutex};

pub struct Audio {
    _stream: cpal::Stream,
    volume: Arc<Mutex<f32>>,
    config: StreamConfig,
}

impl Audio {
    pub fn new<F>(mut sample_callback: F) -> Self
    where
        F: FnMut() -> f32 + Send + 'static,
    {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("No output device available");

        println!("Outputting to device {:?}", device.name());

        let mut config: StreamConfig = device.default_output_config()
            .expect("Failed to get default output config")
            .config();

        config.buffer_size = cpal::BufferSize::Default;

        println!("Initializing new audio device with a sample rate of {}, {} channels, and a buffer size of {:?}", config.sample_rate.0, config.channels, config.buffer_size);

        let volume = Arc::new(Mutex::new(0.5));
        let volume_clone = volume.clone();

        let err_fn = move |err: cpal::StreamError | {
            eprintln!("Stream error: {}", err);
        };

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let current_volume = *volume_clone.lock().unwrap();
                for frame in data.chunks_mut(config.channels as usize) {
                    let sample = sample_callback() * current_volume;

                    for sample_slice in frame.iter_mut() {
                        *sample_slice = sample;
                    }
                }
            },
            err_fn,
            None
        ).expect("Failed to build output stream");

        stream.play().unwrap();

        Self {
            _stream: stream,
            volume,
            config,
        }
    }

    pub fn set_volume(&self, new_volume: f32) {
        let mut volume = self.volume.lock().unwrap();
        *volume = new_volume.clamp(0.0, 1.0);
    }

    pub fn get_volume(&self) -> f32 {
        *self.volume.lock().unwrap()
    }
    
    pub fn get_sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }
}
