//! Core audio extraction and transcoding logic.
//!
//! Uses Symphonia for demuxing and decoding, then pipes decoded
//! samples through a format-specific [`AudioEncoder`].
//! Input is raw bytes; output is encoded bytes.

use std::io::Cursor;

use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

use crate::config::AudioConfig;
use crate::error::{Result, TrackExError};
use crate::format;

/// Extract the first audio track from `config.input_data` and transcode it
/// to `config.format`, returning the encoded audio bytes.
///
/// # Errors
///
/// Returns [`TrackExError::NoAudioTrack`] if the media file has no audio,
/// [`TrackExError::MissingCodecParams`] if codec information is absent,
/// and [`TrackExError::Encoding`] for encoder-specific failures.
pub fn extract_audio(config: &AudioConfig) -> Result<Vec<u8>> {
    // --- 1. Open the media source from memory ---
    let cursor = Cursor::new(&config.input_data);
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

    // --- 2. Probe the format ---
    let hint = symphonia::core::formats::probe::Hint::new();

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

    // --- 6. Create the output encoder ---
    let mut encoder = format::create_encoder(config.format, sample_rate, channels)?;

    // --- 7. Demux + decode loop ---
    let mut sample_buf: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => break,
            Err(SymphoniaError::ResetRequired) => break,
            Err(err) => return Err(err.into()),
        };

        if packet.track_id != track_id {
            continue;
        }

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                let num_samples = audio_buf.samples_interleaved();
                sample_buf.resize(num_samples, 0.0f32);
                audio_buf.copy_to_slice_interleaved(&mut sample_buf);
                encoder.encode(&sample_buf)?;
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(SymphoniaError::IoError(e)) => return Err(e.into()),
            Err(err) => return Err(err.into()),
        }
    }
    // --- 8. Finalize the encoder and return bytes ---
    let encoded = encoder.finalize()?;
    Ok(encoded)
}
