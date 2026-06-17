//! FLAC (Free Lossless Audio Codec) encoder.
//!
//! Requires the `flac` cargo feature. Uses the [`oxideav-flac`] crate,
//! a Pure-Rust FLAC codec + native container implementation.

use std::path::{Path, PathBuf};

use oxideav_core::frame::{AudioFrame, Frame};
use oxideav_core::registry::codec::Encoder as CoreEncoder;
use oxideav_core::registry::container::Muxer;
use oxideav_core::stream::{CodecId, CodecParameters};
use oxideav_core::SampleFormat;

use crate::error::{Result, TrackExError};
use crate::format::AudioEncoder;

/// Encodes interleaved `f32` samples to a FLAC file.
///
/// Samples are accumulated in memory during [`encode`](AudioEncoder::encode)
/// and written to disk in [`finalize`](AudioEncoder::finalize).
pub struct FlacEncoder {
    /// Interleaved `i16` samples.
    samples: Vec<i16>,
    sample_rate: u32,
    channels: u16,
    path: PathBuf,
}

impl FlacEncoder {
    /// Create a new FLAC encoder writing to `path`.
    pub fn new(path: &Path, sample_rate: u32, channels: u16) -> Result<Self> {
        Ok(Self {
            samples: Vec::new(),
            sample_rate,
            channels,
            path: path.to_path_buf(),
        })
    }
}

impl AudioEncoder for FlacEncoder {
    fn encode(&mut self, samples: &[f32]) -> Result<()> {
        for &s in samples {
            let clamped = s.clamp(-1.0, 1.0);
            self.samples.push((clamped * i16::MAX as f32) as i16);
        }
        Ok(())
    }

    fn finalize(self: Box<Self>) -> Result<()> {
        // --- Set up registries ---
        let mut codecs = oxideav_core::registry::codec::CodecRegistry::new();
        let mut containers = oxideav_core::registry::container::ContainerRegistry::new();
        oxideav_flac::register_codecs(&mut codecs);
        oxideav_flac::register_containers(&mut containers);

        // --- Create FLAC encoder ---
        let mut params = CodecParameters::audio(CodecId::new("flac"));
        params.channels = Some(self.channels);
        params.sample_rate = Some(self.sample_rate);
        params.sample_format = Some(SampleFormat::S16);

        let mut enc = codecs
            .first_encoder(&params)
            .map_err(|e| TrackExError::Encoding(format!("FLAC: failed to create encoder: {e}")))?;

        // --- Create FLAC muxer ---
        let out = std::fs::File::create(&self.path)?;
        let streams = vec![oxideav_core::stream::StreamInfo {
            index: 0,
            time_base: oxideav_core::time::TimeBase::new(1, self.sample_rate as i64),
            duration: None,
            start_time: Some(0),
            params: enc.output_params().clone(),
        }];

        let mut mux = containers
            .open_muxer("flac", Box::new(out), &streams)
            .map_err(|e| TrackExError::Encoding(format!("FLAC: failed to create muxer: {e}")))?;

        mux.write_header()
            .map_err(|e| TrackExError::Encoding(format!("FLAC: muxer write_header: {e}")))?;

        // --- Encode frames ---
        let samples_per_channel = self.samples.len() / self.channels as usize;
        let frame_size = 4096; // FLAC fixed block size
        let channel_count = self.channels as usize;

        for frame_start in (0..samples_per_channel).step_by(frame_size) {
            let frame_end = (frame_start + frame_size).min(samples_per_channel);
            let count = frame_end - frame_start;

            // Build interleaved i16 → raw little-endian bytes
            let mut data = Vec::with_capacity(count * channel_count * 2);
            for i in 0..count {
                for ch in 0..channel_count {
                    let sample = self.samples[(frame_start + i) * channel_count + ch];
                    data.extend_from_slice(&sample.to_le_bytes());
                }
            }

            let frame = Frame::Audio(AudioFrame {
                samples: count as u32,
                pts: None,
                data: vec![data],
            });

            enc.send_frame(&frame)
                .map_err(|e| TrackExError::Encoding(format!("FLAC: send_frame: {e}")))?;

            // Drain encoded packets to muxer
            drain_packets(&mut *enc, &mut *mux)?;
        }

        // --- Flush ---
        enc.flush()
            .map_err(|e| TrackExError::Encoding(format!("FLAC: flush: {e}")))?;
        drain_packets(&mut *enc, &mut *mux)?;

        mux.write_trailer()
            .map_err(|e| TrackExError::Encoding(format!("FLAC: muxer write_trailer: {e}")))?;

        Ok(())
    }
}

/// Drain all available packets from the encoder into the muxer.
fn drain_packets(enc: &mut dyn CoreEncoder, mux: &mut dyn Muxer) -> Result<()> {
    loop {
        match enc.receive_packet() {
            Ok(pkt) => {
                mux.write_packet(&pkt)
                    .map_err(|e| TrackExError::Encoding(format!("FLAC: write_packet: {e}")))?;
            }
            Err(_e) => {
                // No more packets available — stop draining.
                break;
            }
        }
    }
    Ok(())
}
