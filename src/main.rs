use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use rubato::{FftFixedInOut, Resampler};
use shutil::pipe;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

const SECONDS: usize = 8;
const BUFFER_SIZE: usize = 44_100; // Depends directly on the sampling frequency of your microphone.
const WHISPER_BUFFER_SIZE: usize = 16_161;

#[derive(Parser, Debug)]
#[command(version, about = "Protect yourself from VI using Gen-AI", long_about = None)]
struct Opt {
    /// If you have multiple input devices and require a specific one, specify it here
    #[arg(short, long, default_value_t = String::from("default"))]
    device: String,
    /// Optional execution mode to simply list the audio devices on the current computer
    #[arg(short, long, default_value_t = false)]
    list_devices: bool,
    /// If you wish to create a running log of all of your audio converted to text provide a log file path
    #[arg(short, long)]
    out_file: Option<String>,
    /// This is your codeword. If the model detects you have said this it will tempt to close all VI instances.
    #[arg(short, long, default_value_t = String::from("the octopus has escaped"))]
    code_phrase: String,
    /// Specify your OpenWhisper model path. A list of models is available here: https://huggingface.co/ggerganov/whisper.cpp/tree/main
    #[arg(short, long, default_value_t = String::from("ggml-base.en.bin"))]
    model_path: String,
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();
    let host = cpal::default_host();

    if opt.list_devices {
        println!("Polling input devices...");
        let mut devices: Vec<String> = vec![];
        for device in host.input_devices()? {
            if let Ok(device_name) = device.name() {
                devices.push(device_name);
            }
        }
        println!("Devices:");
        for (idx, device) in devices.iter().enumerate() {
            println!("\t {idx}. {device}");
        }
        return Ok(());
    }

    // Set up the input device and stream with the default input config.
    let device: Device = if opt.device == "default" {
        host.default_input_device()
    } else {
        host.input_devices()?
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find input device");

    println!("Input device: {}", device.name()?);
    println!("Device configuration: {:?}", device.default_input_config());

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // Inference thread
    let (inference_tx, inference_rx): (Sender<Vec<f32>>, Receiver<Vec<f32>>) = mpsc::channel();
    let inference_thread_handle = thread::spawn(move || {
        let mut maybe_log_file = None;
        if let Some(log_file) = &opt.out_file {
            let out_path: &Path = Path::new(log_file);
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(out_path)
                .unwrap();
            maybe_log_file = Some(file);
        }

        let whisper_ctx =
            WhisperContext::new_with_params(&opt.model_path, WhisperContextParameters::default())
                .expect("failed to load model");

        while let Ok(audio) = inference_rx.recv() {
            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
            params.set_language(Some("english"));
            params.set_debug_mode(false);
            params.set_print_special(false);
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);

            let mut state = whisper_ctx.create_state().expect("failed to create state");
            state
                .full(params, &audio.as_slice())
                .expect("failed to run model");

            let mut transcription_chunk: String = String::new();
            let num_segments = state
                .full_n_segments()
                .expect("failed to get number of segments");

            for i in 0..num_segments {
                let segment = state
                    .full_get_segment_text(i)
                    .expect("failed to get segment");

                if let Some(ref mut file) = maybe_log_file {
                    let start_timestamp = state
                        .full_get_segment_t0(i)
                        .expect("failed to get segment start timestamp");
                    let end_timestamp = state
                        .full_get_segment_t1(i)
                        .expect("failed to get segment end timestamp");

                    writeln!(
                        file,
                        "[{:?} | {} - {}]: {}",
                        chrono::offset::Utc::now(),
                        start_timestamp,
                        end_timestamp,
                        segment
                    )
                    .unwrap();
                }

                transcription_chunk.push_str(segment.as_str());
            }

            if transcription_chunk
                .to_lowercase()
                .contains(&opt.code_phrase.to_lowercase())
            {
                // runs the following command to kill vi!
                // ps aux | grep -ie " vi"  | tr -s ' ' | cut -d ' ' -f 2 | xargs kill -9
                if let Err(error) = pipe(vec![
                    vec!["ps", "aux"],
                    vec!["grep", "-ie", " vi"],
                    vec!["tr", "-s", " "],
                    vec!["cut", "-d", " ", "-f", "2"],
                    vec!["xargs", "kill", "-9"],
                ]) {
                    eprintln!("Shell Command error: {}", error);
                }
            }
        }
    });

    // Streaming data thread
    let (audio_tx, audio_rx): (Sender<Vec<f32>>, Receiver<Vec<f32>>) = mpsc::channel();
    let inference_tx_clone = inference_tx.clone();
    let data_thread_handle = thread::spawn(move || {
        let mut buffer: AllocRingBuffer<f32> = AllocRingBuffer::new(BUFFER_SIZE * SECONDS);
        let mut sample_count = 0;

        // OpenWhisper requires 16000Hz single channel audo
        let mut resampler = FftFixedInOut::<f32>::new(
            BUFFER_SIZE * SECONDS,
            WHISPER_BUFFER_SIZE * SECONDS,
            BUFFER_SIZE * SECONDS,
            1,
        )
        .unwrap();

        while let Ok(samples) = audio_rx.recv() {
            sample_count += samples.len();
            for sample in samples.iter() {
                buffer.push(*sample);
            }

            if sample_count >= BUFFER_SIZE * SECONDS {
                sample_count = 0;
                let vov = vec![buffer.to_vec()];
                let downsample = resampler.process(&vov, None).unwrap();
                inference_tx_clone.send(downsample[0].clone()).unwrap();
            }
        }
    });

    let audio_tx_copy = Arc::new(audio_tx.clone());
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| buffer_stereo_data(data, audio_tx_copy.clone()),
            move |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };

    stream.play()?;

    let mut _buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut _buffer)
        .expect("Failed to read line");

    // Clean up
    drop(stream);
    drop(audio_tx);
    drop(inference_tx);

    data_thread_handle.join().unwrap();
    inference_thread_handle.join().unwrap();
    Ok(())
}

fn buffer_stereo_data(input: &[f32], audio_tx: Arc<Sender<Vec<f32>>>) {
    let downsample = whisper_rs::convert_stereo_to_mono_audio(&input.to_vec())
        .expect("failed to convert audio data");
    if let Err(e) = audio_tx.send(downsample.to_vec()) {
        panic!("Failed to send sample down channel: {}", e);
    }
}
