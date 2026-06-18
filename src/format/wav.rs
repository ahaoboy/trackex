//! WAV (PCM 16-bit) encoder.
//!
//! Always available — does not require any feature flag.
//! Builds a valid 16-bit PCM WAV from accumulated `i16` samples.

use crate::error::Result;
use crate::format::AudioEncoder;

/// Encodes interleaved `f32` samples to a 16-bit PCM WAV buffer.
pub struct WavEncoder {
    /// Interleaved `i16` samples.
    samples: Vec<i16>,
    sample_rate: u32,
    channels: u16,
}

impl WavEncoder {
    /// Create a new WAV encoder.
    pub fn new(sample_rate: u32, channels: u16) -> Result<Self> {
        Ok(Self {
            samples: Vec::new(),
            sample_rate,
            channels,
        })
    }
}

impl AudioEncoder for WavEncoder {
    fn encode(&mut self, samples: &[f32]) -> Result<()> {
        for &s in samples {
            let clamped = s.clamp(-1.0, 1.0);
            self.samples.push((clamped * i16::MAX as f32) as i16);
        }
        Ok(())
    }

    fn finalize(self: Box<Self>) -> Result<Vec<u8>> {
        let data_size = (self.samples.len() * 2) as u32; // i16 = 2 bytes
        let file_size = 44 + data_size; // standard PCM header is 44 bytes

        let mut wav = Vec::with_capacity(file_size as usize);

        // RIFF header
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(file_size - 8).to_le_bytes());
        wav.extend_from_slice(b"WAVE");

        // fmt chunk (16 bytes)
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
        wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
        wav.extend_from_slice(&self.channels.to_le_bytes());
        wav.extend_from_slice(&self.sample_rate.to_le_bytes());
        let byte_rate = self.sample_rate * self.channels as u32 * 2;
        wav.extend_from_slice(&byte_rate.to_le_bytes());
        let block_align = self.channels * 2;
        wav.extend_from_slice(&block_align.to_le_bytes());
        wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

        // data chunk
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_size.to_le_bytes());
        for sample in &self.samples {
            wav.extend_from_slice(&sample.to_le_bytes());
        }

        Ok(wav)
    }
}
