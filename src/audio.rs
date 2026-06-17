//! Core audio extraction and transcoding logic.
//!
//! Uses Symphonia for demuxing and decoding, then pipes decoded
//! samples through a format-specific [`AudioEncoder`].

use std::fs::File;
use std::io::{self, Write};

use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

use crate::config::AudioConfig;
use crate::error::{Result, TrackExError};
use crate::format;

/// Extract the first audio track from `config.input` and transcode it
/// to `config.format`, writing the result to `config.output`.
///
/// # Errors
///
/// Returns [`TrackExError::NoAudioTrack`] if the media file has no audio,
/// [`TrackExError::MissingCodecParams`] if codec information is absent,
/// and [`TrackExError::Encoding`] for encoder-specific failures.
pub fn extract_audio(config: &AudioConfig) -> Result<()> {
    // --- 1. Open the media source ---
    let file = File::open(&config.input)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // --- 2. Probe the format ---
    let mut hint = Hint::new();
    if let Some(ext) = config.input.extension().and_then(|s| s.to_str()) {
        hint.with_extension(ext);
    }

    let mut format = symphonia::default::get_probe().probe(
        &hint,
        mss,
        FormatOptions::default(),
        MetadataOptions::default(),
    )?;

    // --- 3. Locate the first audio track ---
    let track = format
        .first_track(TrackType::Audio)
        .ok_or(TrackExError::NoAudioTrack)?;

    let codec_params = track
        .codec_params
        .as_ref()
        .ok_or(TrackExError::MissingCodecParams)?;

    let audio_params = codec_params
        .audio()
        .ok_or(TrackExError::MissingAudioParams)?;

    // --- 4. Create the audio decoder ---
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(audio_params, &AudioDecoderOptions::default())?;

    let track_id = track.id;

    // --- 5. Determine output audio properties ---
    let sample_rate = config
        .sample_rate
        .unwrap_or_else(|| audio_params.sample_rate.unwrap_or(44100));

    let channels = config.channels.unwrap_or_else(|| {
        audio_params
            .channels
            .as_ref()
            .map_or(2, |c| c.count() as u16)
    });

    println!(
        "Input:  sample_rate={}Hz  channels={}",
        audio_params.sample_rate.unwrap_or(0),
        audio_params.channels.as_ref().map_or(0, |c| c.count())
    );
    println!(
        "Output: sample_rate={sample_rate}Hz  channels={channels}  format={:?}",
        config.format
    );

    // --- 6. Create the output encoder ---
    let mut encoder = format::create_encoder(config.format, &config.output, sample_rate, channels)?;

    // --- 7. Demux + decode loop ---
    let mut sample_buf: Vec<f32> = Vec::new();
    let mut total_frames: u64 = 0;

    loop {
        let packet = match format.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => break, // end of stream
            Err(SymphoniaError::ResetRequired) => break,
            Err(err) => {
                eprintln!("packet read error: {err}");
                break;
            }
        };

        // Skip non-audio packets (e.g. video frames)
        if packet.track_id != track_id {
            continue;
        }

        // Consume any new metadata
        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        // Decode
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                let num_samples = audio_buf.samples_interleaved();
                sample_buf.resize(num_samples, 0.0f32);

                // Convert decoded audio to interleaved f32
                audio_buf.copy_to_slice_interleaved(&mut sample_buf);

                // Send to encoder
                encoder.encode(&sample_buf)?;

                total_frames += audio_buf.frames() as u64;
                print!("\rDecoded {total_frames} frames...");
                io::stdout().flush()?;
            }
            Err(SymphoniaError::DecodeError(_)) => {
                // Skip packets that fail to decode
                continue;
            }
            Err(SymphoniaError::IoError(_)) => {
                continue;
            }
            Err(err) => {
                eprintln!("decode error: {err}");
                break;
            }
        }
    }

    println!(); // newline after progress

    // --- 8. Finalize the encoder ---
    encoder.finalize()?;

    println!("Successfully wrote {}", config.output.display());
    Ok(())
}
