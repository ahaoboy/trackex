//! MP3 (MPEG-1 Audio Layer 3) encoder.
//!
//! Requires the `mp3` cargo feature. Uses the OxiAudio LAME adapter
//! (`mp3-encode-lame`) to encode via libmp3lame.

use oxiaudio::{AudioBuffer, ChannelLayout, SampleFormat};

use crate::error::{Result, TrackExError};
use crate::format::AudioEncoder;

/// Encodes interleaved `f32` samples to an MP3 buffer via LAME.
pub struct Mp3Encoder {
    /// Accumulated interleaved `f32` samples.
    samples: Vec<f32>,
    sample_rate: u32,
    channels: u16,
}

impl Mp3Encoder {
    /// Create a new MP3 encoder.
    pub fn new(sample_rate: u32, channels: u16) -> Result<Self> {
        Ok(Self {
            samples: Vec::new(),
            sample_rate,
            channels,
        })
    }
}

impl AudioEncoder for Mp3Encoder {
    fn encode(&mut self, samples: &[f32]) -> Result<()> {
        self.samples.extend_from_slice(samples);
        Ok(())
    }

    fn finalize(self: Box<Self>) -> Result<Vec<u8>> {
        let buf = AudioBuffer {
            samples: self.samples,
            sample_rate: self.sample_rate,
            channels: ChannelLayout::from(self.channels),
            format: SampleFormat::F32,
        };

        // Write to temp file, then read back.
        // (oxiaudio's MP3 encoder writes to a path; no direct Vec<u8> API.)
        let tmp = std::env::temp_dir().join(format!("trackex_mp3_{}.tmp", std::process::id()));
        oxiaudio::encode_mp3_with_auto_replaygain(&buf, &tmp, 192)
            .map_err(|e| TrackExError::Encoding(format!("MP3 encode error: {e}")))?;

        let data = std::fs::read(&tmp)?;
        let _ = std::fs::remove_file(&tmp);
        Ok(data)
    }
}
