use num_complex::Complex;
use rfnm::device::Device;
use rfnm::stream::RxStream;
use rfnm_sys::rfnm_channel;
use std::error::Error;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Setting up device etc");
    let device = Device::connect_usb()?;
    eprintln!("Device created successfully");
    let stream =
        RxStream::<Complex<f32>>::new(device, rfnm_channel::CH0).expect("Could not create stream");
    eprintln!(
        "Connected and ready to stream. Device suggests a buffer with {} elements in it.",
        stream.suggested_buffer_size()
    );
    let mut scratch = vec![Complex::new(0.0, 0.0); stream.suggested_buffer_size()];
    let buffers = [scratch.as_mut_slice()];
    // before we start, lets make sure we are somewhat sanely setup though
    let ch_settings = stream
        .device()
        .get_rx_settings(rfnm_channel::CH0)?
        .to_settings();
    if let Err(e) = stream
        .device()
        .set_rx_settings(rfnm_channel::CH0, &ch_settings)
    {
        eprintln!("Could not setup channel: {e}");
        return Err(e.into());
    }
    if let Err(e) = stream.start() {
        eprintln!("Could not start streaming: {e}");
        return Err(e.into());
    }

    let mut total_samples = 0;
    let mut since_last = 0;
    let started = Instant::now();
    let mut last = started;
    loop {
        match stream.read(buffers.as_slice(), Duration::from_millis(10)) {
            Ok(info) => {
                total_samples += info.elements_read;
                since_last += info.elements_read;
                let now = Instant::now();
                if now.duration_since(last) > Duration::from_secs(5) {
                    let rate_since_start =
                        total_samples as f64 / now.duration_since(started).as_secs_f64();
                    let rate_since_last =
                        since_last as f64 / now.duration_since(last).as_secs_f64();
                    eprintln!(
                        "Samples since last status: {since_last}, that is {rate_since_last}/s"
                    );
                    eprintln!("Total samples: {total_samples}, that is {rate_since_start}/s");
                    last = now;
                    since_last = 0;
                }
            }
            Err(e) => {
                eprintln!("Error while streaming: {e}");
                return Err(e.into());
            }
        }
    }
}
