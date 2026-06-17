//! Configuration types for audio extraction.
//!
//! The [`AudioConfig`] struct holds all parameters needed to extract
//! and convert audio. It supports optional serde serialization.

use std::path::PathBuf;

use crate::format::OutputFormat;

/// Configuration for an audio extraction job.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use trackex::config::AudioConfig;
/// use trackex::format::OutputFormat;
///
/// let cfg = AudioConfig {
///     input: PathBuf::from("video.mp4"),
///     output: PathBuf::from("audio.wav"),
///     format: OutputFormat::Wav,
///     sample_rate: None,
///     channels: None,
/// };
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AudioConfig {
    /// Path to the input media file (video or audio).
    pub input: PathBuf,

    /// Path where the extracted audio will be written.
    pub output: PathBuf,

    /// Desired output format (WAV, MP3, OGG, …).
    pub format: OutputFormat,

    /// Optional target sample rate in Hz.
    /// When `None` the original sample rate is preserved.
    pub sample_rate: Option<u32>,

    /// Optional target channel count.
    /// When `None` the original channel count is preserved.
    /// Note: channel remapping/resampling is not yet implemented.
    pub channels: Option<u16>,
}
