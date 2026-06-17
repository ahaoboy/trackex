//! MP3 (MPEG-1 Audio Layer 3) encoder.
//!
//! Requires the `mp3` cargo feature.
//! Currently a stub — real LAME encoding is not yet implemented.

use std::path::Path;

use crate::error::Result;
use crate::format::AudioEncoder;

/// Encodes interleaved `f32` samples to an MP3 file.
pub struct Mp3Encoder;

impl Mp3Encoder {
    /// Create a new MP3 encoder writing to `path`.
    pub fn new(_path: &Path, _sample_rate: u32, _channels: u16) -> Result<Self> {
        todo!("MP3 encoding is not yet implemented")
    }
}

impl AudioEncoder for Mp3Encoder {
    fn encode(&mut self, _samples: &[f32]) -> Result<()> {
        todo!("MP3 encoding is not yet implemented")
    }

    fn finalize(self: Box<Self>) -> Result<()> {
        todo!("MP3 encoding is not yet implemented")
    }
}
