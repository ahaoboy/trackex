//! Ogg Vorbis encoder.
//!
//! Requires the `ogg` cargo feature. Uses the [`vorbis-encoder`] crate
//! (FFI to libvorbis + libogg) for both Vorbis codec encoding and Ogg
//! container muxing.
//!
//! ## Migration plan
//!
//! The long-term goal is to replace `vorbis-encoder` with the Pure-Rust
//! `oxideav-ogg` (Ogg container) + `oxideav-vorbis` (Vorbis codec) once
//! the latter's encoder pipeline is complete (currently decoder-only).
//! `vorbis-encoder` takes interleaved `i16` input; we accumulate samples
//! in memory and encode them in [`AudioEncoder::finalize`].

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::error::{Result, TrackExError};
use crate::format::AudioEncoder;

/// Encodes interleaved `f32` samples to an Ogg Vorbis file.
///
/// Samples are accumulated in memory during [`encode`](AudioEncoder::encode)
/// and written to disk in [`finalize`](AudioEncoder::finalize).
pub struct OggEncoder {
    /// Interleaved `i16` samples.
    samples: Vec<i16>,
    sample_rate: u32,
    channels: u16,
    path: PathBuf,
}

impl OggEncoder {
    /// Create a new Ogg Vorbis encoder writing to `path`.
    pub fn new(path: &Path, sample_rate: u32, channels: u16) -> Result<Self> {
        Ok(Self {
            samples: Vec::new(),
            sample_rate,
            channels,
            path: path.to_path_buf(),
        })
    }
}

impl AudioEncoder for OggEncoder {
    fn encode(&mut self, samples: &[f32]) -> Result<()> {
        for &s in samples {
            let clamped = s.clamp(-1.0, 1.0);
            self.samples.push((clamped * i16::MAX as f32) as i16);
        }
        Ok(())
    }

    fn finalize(self: Box<Self>) -> Result<()> {
        let mut enc = vorbis_encoder::Encoder::new(
            self.channels as u32,
            self.sample_rate as u64,
            0.4, // quality ∈ [0.0, 1.0]; 0.4 ≈ medium
        )
        .map_err(|code| {
            TrackExError::Encoding(format!(
                "failed to create Vorbis encoder (error code: {code})"
            ))
        })?;

        let chunk_samples = 4096 * self.channels as usize;
        let mut file = BufWriter::new(File::create(&self.path)?);

        for chunk in self.samples.chunks(chunk_samples) {
            let data = enc.encode(&chunk.to_vec()).map_err(|code| {
                TrackExError::Encoding(format!("Vorbis encode error (code: {code})"))
            })?;
            file.write_all(&data)?;
        }

        // Flush remaining data.
        let tail = enc
            .flush()
            .map_err(|code| TrackExError::Encoding(format!("Vorbis flush error (code: {code})")))?;

        file.write_all(&tail)?;
        file.flush()?;

        Ok(())
    }
}
