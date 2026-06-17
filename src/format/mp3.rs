//! MP3 (MPEG-1 Audio Layer 3) encoder.
//!
//! Requires the `mp3` cargo feature. Uses the OxiAudio LAME adapter
//! (`mp3-encode-lame`) to encode via libmp3lame.

use std::path::{Path, PathBuf};

use oxiaudio::{AudioBuffer, ChannelLayout, SampleFormat};

use crate::error::{Result, TrackExError};
use crate::format::AudioEncoder;

/// Encodes interleaved `f32` samples to an MP3 file via LAME.
pub struct Mp3Encoder {
    /// Accumulated interleaved `f32` samples.
    samples: Vec<f32>,
    sample_rate: u32,
    channels: u16,
    path: PathBuf,
}

impl Mp3Encoder {
    /// Create a new MP3 encoder writing to `path`.
    pub fn new(path: &Path, sample_rate: u32, channels: u16) -> Result<Self> {
        Ok(Self {
            samples: Vec::new(),
            sample_rate,
            channels,
            path: path.to_path_buf(),
        })
    }
}

impl AudioEncoder for Mp3Encoder {
    fn encode(&mut self, samples: &[f32]) -> Result<()> {
        self.samples.extend_from_slice(samples);
        Ok(())
    }

    fn finalize(self: Box<Self>) -> Result<()> {
        let buf = AudioBuffer {
            samples: self.samples,
            sample_rate: self.sample_rate,
            channels: ChannelLayout::from(self.channels),
            format: SampleFormat::F32,
        };

        // VBR "Music" preset at 192 kbps nominal, writes directly to file.
        oxiaudio::encode_mp3_with_auto_replaygain(&buf, &self.path, 192)
            .map_err(|e| TrackExError::Encoding(format!("MP3 encode error: {e}")))?;

        Ok(())
    }
}
