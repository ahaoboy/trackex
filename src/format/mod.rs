//! Output format abstraction.
//!
//! Each supported audio format implements the [`AudioEncoder`] trait.
//! The [`OutputFormat`] enum maps to the appropriate encoder implementation
//! and can be used directly with clap's `ValueEnum` derive.

#[cfg(feature = "wav")]
pub mod wav;

#[cfg(feature = "mp3")]
pub mod mp3;

#[cfg(feature = "ogg")]
pub mod ogg;

#[cfg(feature = "flac")]
pub mod flac;

use crate::error::Result;

/// Supported output formats.
///
/// Variants may be gated behind cargo features so that only the
/// formats the user actually needs are compiled in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OutputFormat {
    /// Microsoft WAV (PCM 16-bit). Requires the `wav` feature.
    #[cfg(feature = "wav")]
    #[cfg_attr(feature = "cli", clap(name = "wav"))]
    Wav,

    /// MPEG-1 Audio Layer 3. Requires the `mp3` feature.
    #[cfg(feature = "mp3")]
    #[cfg_attr(feature = "cli", clap(name = "mp3"))]
    Mp3,

    /// Ogg Vorbis. Requires the `ogg` feature.
    #[cfg(feature = "ogg")]
    #[cfg_attr(feature = "cli", clap(name = "ogg"))]
    Ogg,

    /// FLAC lossless. Requires the `flac` feature.
    #[cfg(feature = "flac")]
    #[cfg_attr(feature = "cli", clap(name = "flac"))]
    Flac,
}

/// Common interface for audio encoders.
///
/// Each encoder receives interleaved `f32` samples (range -1.0 … 1.0)
/// and produces encoded bytes in [`finalize`](AudioEncoder::finalize).
pub trait AudioEncoder: Send {
    /// Encode a block of interleaved `f32` samples.
    fn encode(&mut self, samples: &[f32]) -> Result<()>;

    /// Flush any remaining data and return the encoded audio bytes.
    fn finalize(self: Box<Self>) -> Result<Vec<u8>>;
}

impl OutputFormat {
    /// Infer the output format from a file extension.
    ///
    /// Supports: `wav`, `mp3`, `ogg` (case-insensitive).
    /// Returns an error if the extension is not recognised.
    pub fn from_extension(ext: &str) -> Result<Self> {
        match ext.to_lowercase().as_str() {
            #[cfg(feature = "wav")]
            "wav" => Ok(Self::Wav),
            #[cfg(not(feature = "wav"))]
            "wav" => Err(crate::error::TrackExError::UnsupportedFormat(
                "WAV support is not compiled in — enable the `wav` feature".into(),
            )),

            #[cfg(feature = "mp3")]
            "mp3" => Ok(Self::Mp3),
            #[cfg(not(feature = "mp3"))]
            "mp3" => Err(crate::error::TrackExError::UnsupportedFormat(
                "MP3 support is not compiled in — enable the `mp3` feature".into(),
            )),

            #[cfg(feature = "ogg")]
            "ogg" => Ok(Self::Ogg),
            #[cfg(not(feature = "ogg"))]
            "ogg" => Err(crate::error::TrackExError::UnsupportedFormat(
                "OGG support is not compiled in — enable the `ogg` feature".into(),
            )),

            #[cfg(feature = "flac")]
            "flac" => Ok(Self::Flac),
            #[cfg(not(feature = "flac"))]
            "flac" => Err(crate::error::TrackExError::UnsupportedFormat(
                "FLAC support is not compiled in — enable the `flac` feature".into(),
            )),

            other => Err(crate::error::TrackExError::UnknownExtension(
                other.to_owned(),
            )),
        }
    }
}

/// Factory: create the right encoder for the given format.
pub fn create_encoder(
    format: OutputFormat,
    sample_rate: u32,
    channels: u16,
) -> Result<Box<dyn AudioEncoder>> {
    match format {
        #[cfg(feature = "wav")]
        OutputFormat::Wav => {
            let enc = wav::WavEncoder::new(sample_rate, channels)?;
            Ok(Box::new(enc))
        }
        #[cfg(feature = "mp3")]
        OutputFormat::Mp3 => {
            let enc = mp3::Mp3Encoder::new(sample_rate, channels)?;
            Ok(Box::new(enc))
        }
        #[cfg(feature = "ogg")]
        OutputFormat::Ogg => {
            let enc = ogg::OggEncoder::new(sample_rate, channels)?;
            Ok(Box::new(enc))
        }
        #[cfg(feature = "flac")]
        OutputFormat::Flac => {
            let enc = flac::FlacEncoder::new(sample_rate, channels)?;
            Ok(Box::new(enc))
        }
    }
}
