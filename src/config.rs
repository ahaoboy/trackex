//! Configuration types for audio extraction.
//!
//! The [`AudioConfig`] struct holds all parameters needed to extract
//! and convert audio. It supports optional serde serialization.

use crate::format::OutputFormat;

/// Configuration for an audio extraction job.
///
/// # Examples
///
/// ```no_run
/// use trackex::config::AudioConfig;
/// use trackex::format::OutputFormat;
///
/// let cfg = AudioConfig {
///     input_data: std::fs::read("video.mp4").unwrap(),
///     format: OutputFormat::Wav,
///     sample_rate: None,
///     channels: None,
/// };
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AudioConfig {
    /// Raw bytes of the input media file (video or audio).
    pub input_data: Vec<u8>,

    /// Desired output format (WAV, MP3, OGG, FLAC).
    pub format: OutputFormat,

    /// Optional target sample rate in Hz.
    /// When `None` the original sample rate is preserved.
    pub sample_rate: Option<u32>,

    /// Optional target channel count.
    /// When `None` the original channel count is preserved.
    pub channels: Option<u16>,
}
