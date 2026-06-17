//! Ogg Vorbis encoder.
//!
//! Requires the `ogg` cargo feature.
//! Currently a stub — real Vorbis encoding is not yet implemented.

use std::path::Path;

use crate::error::Result;
use crate::format::AudioEncoder;

/// Encodes interleaved `f32` samples to an Ogg Vorbis file.
pub struct OggEncoder;

impl OggEncoder {
    /// Create a new Ogg Vorbis encoder writing to `path`.
    pub fn new(_path: &Path, _sample_rate: u32, _channels: u16) -> Result<Self> {
        todo!("Ogg Vorbis encoding is not yet implemented")
    }
}

impl AudioEncoder for OggEncoder {
    fn encode(&mut self, _samples: &[f32]) -> Result<()> {
        todo!("Ogg Vorbis encoding is not yet implemented")
    }

    fn finalize(self: Box<Self>) -> Result<()> {
        todo!("Ogg Vorbis encoding is not yet implemented")
    }
}
