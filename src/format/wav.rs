//! WAV (PCM 16-bit) encoder.
//!
//! Always available — does not require any feature flag.

use std::path::Path;

use crate::error::Result;
use crate::format::AudioEncoder;

/// Encodes interleaved `f32` samples to a 16-bit PCM WAV file.
pub struct WavEncoder {
    inner: hound::WavWriter<std::io::BufWriter<std::fs::File>>,
}

impl WavEncoder {
    /// Create a new WAV encoder writing to `path`.
    pub fn new(path: &Path, sample_rate: u32, channels: u16) -> Result<Self> {
        let spec = hound::WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let writer = hound::WavWriter::create(path, spec)?;
        Ok(Self { inner: writer })
    }
}

impl AudioEncoder for WavEncoder {
    fn encode(&mut self, samples: &[f32]) -> Result<()> {
        for &sample in samples {
            // Clamp to [-1.0, 1.0] and scale to i16 range
            let scaled = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            self.inner.write_sample(scaled)?;
        }
        Ok(())
    }

    fn finalize(self: Box<Self>) -> Result<()> {
        self.inner.finalize()?;
        Ok(())
    }
}
