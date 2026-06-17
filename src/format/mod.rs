//! Output format abstraction.
//!
//! Each supported audio format implements the [`AudioEncoder`] trait.
//! The [`OutputFormat`] enum maps to the appropriate encoder implementation
//! and can be used directly with clap's `ValueEnum` derive.

pub mod wav;

#[cfg(feature = "mp3")]
pub mod mp3;

#[cfg(feature = "ogg")]
pub mod ogg;

use std::path::Path;

use crate::error::Result;

/// Supported output formats.
///
/// Variants may be gated behind cargo features so that only the
/// formats the user actually needs are compiled in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OutputFormat {
    /// Microsoft WAV (PCM 16-bit). Always available.
    Wav,

    /// MPEG-1 Audio Layer 3. Requires the `mp3` feature.
    #[cfg(feature = "mp3")]
    #[cfg_attr(feature = "cli", clap(name = "mp3"))]
    Mp3,

    /// Ogg Vorbis. Requires the `ogg` feature.
    #[cfg(feature = "ogg")]
    #[cfg_attr(feature = "cli", clap(name = "ogg"))]
    Ogg,
}

/// Common interface for audio encoders.
///
/// Each encoder receives interleaved `f32` samples (range -1.0 … 1.0)
/// and writes encoded data to the underlying sink.
pub trait AudioEncoder: Send {
    /// Encode a block of interleaved `f32` samples.
    fn encode(&mut self, samples: &[f32]) -> Result<()>;

    /// Flush any remaining data and finalize the output file.
    fn finalize(self: Box<Self>) -> Result<()>;
}

impl OutputFormat {
    /// Infer the output format from a file extension.
    ///
    /// Supports: `wav`, `mp3`, `ogg` (case-insensitive).
    /// Returns an error if the extension is not recognised.
    pub fn from_extension(ext: &str) -> Result<Self> {
        match ext.to_lowercase().as_str() {
            "wav" => Ok(Self::Wav),
            #[cfg(feature = "mp3")]
            "mp3" => Ok(Self::Mp3),
            #[cfg(feature = "ogg")]
            "ogg" => Ok(Self::Ogg),
            other => Err(crate::error::TrackExError::UnknownExtension(
                other.to_owned(),
            )),
        }
    }
}

/// Factory: create the right encoder for the given format.
pub fn create_encoder(
    format: OutputFormat,
    path: &Path,
    sample_rate: u32,
    channels: u16,
) -> Result<Box<dyn AudioEncoder>> {
    match format {
        OutputFormat::Wav => {
            let enc = wav::WavEncoder::new(path, sample_rate, channels)?;
            Ok(Box::new(enc))
        }
        #[cfg(feature = "mp3")]
        OutputFormat::Mp3 => {
            let enc = mp3::Mp3Encoder::new(path, sample_rate, channels)?;
            Ok(Box::new(enc))
        }
        #[cfg(feature = "ogg")]
        OutputFormat::Ogg => {
            let enc = ogg::OggEncoder::new(path, sample_rate, channels)?;
            Ok(Box::new(enc))
        }
    }
}
